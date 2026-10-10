use super::common::*;
use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn neko_neko_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(2213)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(983)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(914)])?.number()? > 199)
        && ctx.var("Zeny").get()?.number()? > 9999)
    {
        ctx.lines_as(
            "Neko Neko",
            args!["Heh? You brought all the items~", "Okay, give me some time."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Neko Neko",
            args![
                "*Squish Squish Bang Bang Bang Tap Tap Scrape Scrape*",
                "*Scratch Scratch Tap Tap Tap Tap*"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Neko Neko",
            args![
                "*Squish Squish Bang Bang Bang Tap Tap Scrape Scrape*",
                "*Scratch Scratch Tap Tap Tap Tap*"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Neko Neko",
            args!["Phew! There you go.", "Now you can wear", "Black Cat Ears", "of your very own~"],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(2213), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(983), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(914), Val::from(200)])?;
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
        ctx.call(Function::GetItem, vec![Val::from(5057), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as("Neko Neko", args!["Thank you for using my service~!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Neko Neko",
        args![
            "Umm...?",
            "Excuse me?",
            "Do I know you?...",
            "Oh, I know why you",
            "want to talk to me.",
            "Heh heh~!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Neko Neko",
        args!["You want to know where I got these tiny little cute black cat ears on my head, don't you? I knew it! Hahaha~!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Neko Neko",
        args![
            "Well, I made these myself.",
            "If you want, I can make one for you too for a small price.",
            "Heh heh heh~"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Neko Neko",
        args![
            "Just bring me",
            "1 ^FF0000Kitty Band^000000,",
            "1 ^FF0000Black Dyestuffs^000000,",
            "200 ^FF0000Fluff^000000 and",
            "^FF000010,000 zeny^000000!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Neko Neko", args!["^FF0000Just remember, if you bring me any item that has a card inserted, or has been upgraded, the additional abilities of the item will disappear after I have used them. So please keep that in mind."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn neko_neko_1(ctx: &Ctx) -> Script {
    neko_neko_1_body(ctx, Vec::new()).map(|_| ())
}

fn argen_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines_as(
            "Argen",
            args!["Hey~ why are you carrying so many items?", "You mind sharing them with me?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SCRATCH")?])?;
    ctx.lines_as(
        "Argen",
        args![
            "Man, oh man~",
            "Am I bored~!",
            "If I only knew more tricks with this yo-yo. Then I could really impress the ladies."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
        args!["Impress the ladies...?"],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.lines_as(
            "Argen",
            args!["Yeah dude. There are two things chicks dig. Small, adorable presents. And suave dudes."],
        )?;
        ctx.next()?;
        ctx.lines_as("Argen", args!["So, I got the cutesy present junk down. You know, instead of like, spending money on like a real gift, I make these little hairpin thingees."])?;
        ctx.next()?;
        ctx.lines_as("Argen", args!["As for being suave? Well, I do have this yo-yo..."])?;
        ctx.next()?;
        ctx.lines_as("Argen", args!["Heh. You look like you don't like wasting cash on girls either. I'm down with that. If you like, I can make you a hairpin that you can give as gifts to the ^FF3333ladies^000000."])?;
        ctx.next()?;
        ctx.lines_as(
            "Argen",
            args![
                "All you gotta do is bring me the stuff to make it. And if that hairpin's for yourself, well, I'm down with that jive too."
            ],
        )?;
    } else {
        ctx.lines_as("Argen", args!["Whoa~! A lady...", "Wow, you're really cute! Tell you what..."])?;
        ctx.next()?;
        ctx.lines_as("Argen", args!["Since you're such a cutie, I'll put together a nice little hairpin for you. All you gotta do is bring me the stuff to make it."])?;
        ctx.next()?;
        ctx.lines_as(
            "Argen",
            args!["So go ahead and tell me what kinda hairpin you're interested in."],
        )?;
    }
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
    match runtime::select_values(
        ctx,
        &[Val::from(
            "^3131FFX Hairpin^000000:^3131FFBand Aid^000000:^3131FFFlower Hairpin^000000:No thanks.",
        )],
    )? {
        1 => {
            if (ctx.call(Function::CountItem, vec![Val::from(2294)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(7220)])?.number()? > 399)
            {
                ctx.lines_as("Argen", args!["X Hairpin!", "Nice choice~!"])?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
                ctx.lines_as(
                    "Argen",
                    args![
                        "Ah... Right.",
                        "About the Stellar...",
                        "^CE3100If you use an upgraded Stellar to make this item, any upgrades will be lost. That okay with you?^000000"
                    ],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("I don't mind.:I will come back later.")])? {
                    1 => {
                        ctx.lines_as("Argen", args!["Alright, let's get to work."])?;
                        ctx.next()?;
                        ctx.lines(args!["^3131FF * Scrape Scrape * ^000000", "^3131FF * Scrape Scrape * ^000000"])?;
                        ctx.next()?;
                        ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                        ctx.lines_as(
                            "Argen",
                            args!["Alright, I'm done!", "So what do you think?", "Pretty cool, eh?"],
                        )?;
                        ctx.call(Function::DelItem, vec![Val::from(2294), Val::from(1)])?;
                        ctx.call(Function::DelItem, vec![Val::from(7220), Val::from(400)])?;
                        ctx.call(Function::GetItem, vec![Val::from(5079), Val::from(1)])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Argen", args!["No problem, take your time."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else {
                ctx.lines_as("Argen", args!["Oh~", "You want a ^3131FFX Hairpin?^000000", "That's easy!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Argen",
                    args![
                        "I need ^FF00001 Stellar^000000",
                        "and ^FF0000400 Ectoplasm^000000.",
                        "Tell me when you're ready!"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        2 => {
            if (ctx.call(Function::CountItem, vec![Val::from(970)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(930)])?.number()? > 499)
            {
                ctx.lines_as(
                    "Argen",
                    args!["Band Aid, eh?", "Got it! Hold on", "a sec, I'll be", "done in a bit."],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FF * Scrape Scrape * ^000000", "^3131FF * Scrape Scrape * ^000000"])?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                ctx.lines_as("Argen", args!["*Whew~!*", "I'm finished!", "Pretty cool, huh?"])?;
                ctx.call(Function::DelItem, vec![Val::from(970), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(930), Val::from(500)])?;
                ctx.call(Function::GetItem, vec![Val::from(5063), Val::from(1)])?;
                ctx.next()?;
            } else {
                ctx.lines_as(
                    "Argen",
                    args![
                        "Ah, a ^3131FFBand Aid^000000!",
                        "That stuff is great for faking injuries, or hiding a bald spot. Handy to have when you wanna ditch school."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Argen",
                    args![
                        "Bring me ^FF00001 Alcohol^000000 and",
                        "^FF0000500 Rotten Bandage^000000!",
                        "Tell me when you're ready, yeah?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        3 => {
            if ((ctx.call(Function::CountItem, vec![Val::from(2269)])?.number()? > 0
                && ctx.call(Function::CountItem, vec![Val::from(999)])?.number()? > 9)
                && ctx.var("Zeny").get()?.number()? > 19999)
            {
                ctx.lines_as(
                    "Argen",
                    args!["You wanna a Flower Hairpin~?", "Alright, gimmie a minute, yeah?"],
                )?;
                ctx.next()?;
                ctx.lines(args!["^3131FF - Scrape Scrape - ^000000", "^3131FF - Scrape Scrape - ^000000"])?;
                ctx.next()?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_BEST")?])?;
                ctx.lines_as(
                    "Argen",
                    args![
                        "Ah... It's done!",
                        "Hey, that looks pretty smooth! Oh, and uh, you can keep this leftover Stem."
                    ],
                )?;
                ctx.call(Function::DelItem, vec![Val::from(2269), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(999), Val::from(10)])?;
                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                ctx.call(Function::GetItem, vec![Val::from(5061), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(905), Val::from(1)])?;
                ctx.next()?;
            } else {
                ctx.lines_as("Argen", args!["You wanna a ^3131FFFlower Hairpin^000000!", "Yeah, I've noticed cuties just love walking around with flowers in their mouths, and I was always like 'Whaaaat~?' But then I got this great idea!"])?;
                ctx.next()?;
                ctx.lines_as("Argen", args!["Wouldn't some kinda... ^3131FFFlower Hairpin^000000 be a hit with chicks? So I tried making one, and it seemed to be pretty popular among my lady friends."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Argen",
                    args![
                        "And it's a much better way to use a flower than to put it on Pet Orc Warriors. Man, those things just look gnarly~"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Argen", args!["Well, if you want one, I'll make it. But I want you should pay me some royalties for my services. It's pretty hard to attach a flower to a hairpin, you know?"])?;
                ctx.next()?;
                ctx.lines_as("Argen", args!["It takes mad skill."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Argen",
                    args![
                        "So, I want you to bring...",
                        "^FF00001 Romantic Flower^000000,",
                        "^FF000010 Steel^000000",
                        "and ^3131FF20,000 zeny^000000!",
                        "Got it? Cool~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
        4 => {
            ctx.lines_as(
                "Argen",
                args![
                    "I'm down with that.",
                    "If you want some kinda hairpin, just come back. I'll just be here, looking ^FF3333suaaave^000000 with this yo-yo."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    ctx.call(Function::Emotion, vec![ctx.constant("ET_WRAP")?])?;
    ctx.lines_as("Argen", args!["Feel free to come ask me if you want more hairpins. Hey, I know I look lazy, but I do like to make things for the ladies. See ya later~!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn argen_1(ctx: &Ctx) -> Script {
    argen_1_body(ctx, Vec::new()).map(|_| ())
}

fn zhenbolt_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(7216)])?.number()? > 299
        && ctx.call(Function::CountItem, vec![Val::from(7097)])?.number()? > 299)
        && ctx.call(Function::CountItem, vec![Val::from(2211)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(982)])?.number()? > 0)
    {
        ctx.lines_as(
            "Zhenbolt",
            args![
                "What's this?!",
                "You've already brought everything I've asked! Good job, hero~! Let me get to work right away..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Zhenbolt", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Zhenbolt", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("Zhenbolt", args!["......*Cling*"])?;
        ctx.next()?;
        ctx.lines_as("Zhenbolt", args!["Take this...", "It's known as the ^FF0000Hot-blooded Headband^000000, my personal symbol of fighting spirit! With this on your head, nothing can stop you!"])?;
        ctx.call(Function::DelItem, vec![Val::from(7216), Val::from(300)])?;
        ctx.call(Function::DelItem, vec![Val::from(7097), Val::from(300)])?;
        ctx.call(Function::DelItem, vec![Val::from(2211), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(982), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(5070), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as(
            "Zhenbolt",
            args!["Now, go!", "Unleash your fighting spirit upon the villains of the world!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Zhenbolt", args!["I...", "am called ^FF0000Zhenbolt^000000!!"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("THE Zhenbolt!?:Oh my gosh, it's Zhenbolt!")],
    )?) == 1
    {
        ctx.lines_as(
            "Zhenbolt",
            args!["Yes, it is I, the man who has pummelled over a trillion foes and rising. Raging Hurricane Hero of Justice: Zhenbolt!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zhenbolt",
            args!["I see in your eyes that you also fight with a heart of purity, a heart of passion!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Zhenbolt", args!["It is decided! If you bring me the following items, I will give you the famous symbol of battle by which villains fear Zhenbolt."])?;
        ctx.next()?;
        ctx.lines_as(
            "Zhenbolt",
            args![
                "Bring me...",
                "300 ^FF0000Red Muffler^000000,",
                "300 ^FF0000Burning Heart^000000,",
                "1 ^FF0000Bandana^000000 and 1 ^FF0000White Dyestuffs^000000.",
                "That's all I need."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Zhenbolt",
            args!["Come back soon, adventurer!", "I expect great things of you!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Zhenbolt",
        args![
            "Yes.",
            "It is I, Zhenbolt.",
            "Warrior of Might!",
            "Whirling Tempest",
            "of Raging Passion!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zhenbolt",
        args!["I see in your eyes that you have seen many a skirmish. To have survived this long, you must know the secret to victory..."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zhenbolt",
        args!["Pure, unbridled rage!", "Battles are won with vigor and passion!"],
    )?;
    ctx.next()?;
    ctx.lines_as("Zhenbolt", args!["Seeing as you understand this fundamental, I recognize you as an ally in the eternal struggle for justice! I shall bestow upon you the very symbol with which I have struck fear into evil hearts..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Zhenbolt",
        args![
            "Bring me",
            "300 ^FF0000Red Muffler^000000,",
            "300 ^FF0000Burning Heart^000000,",
            "1 ^FF0000Bandana^000000 and 1 ^FF0000White Dyestuffs^000000.",
            "That's all I need."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Zhenbolt",
        args!["Bring me those items...", "And you too can become a fighting legend..."],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn zhenbolt_1(ctx: &Ctx) -> Script {
    zhenbolt_1_body(ctx, Vec::new()).map(|_| ())
}

fn nephia_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ((ctx.call(Function::CountItem, vec![Val::from(2244)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(2209)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(10007)])?.number()? > 0)
    {
        ctx.lines_as(
            "Nephia",
            args![
                "Oh...",
                "You've brought all these ribbons! I can't wait to tie you a new ribbon to put on your ^FF66CCbeautiful hair!^000000"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nephia", args!["..."])?;
        ctx.next()?;
        ctx.lines_as("Nephia", args!["......"])?;
        ctx.next()?;
        ctx.lines_as("Nephia", args![".......*Cling*"])?;
        ctx.next()?;
        ctx.lines_as(
            "Nephia",
            args!["That's it~! We're done! Oh, I just know you're going to look precious wearing this."],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(2244), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(2209), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(10007), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(5083), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as(
            "Nephia",
            args!["Hopefully, we can get together and do each other's hair sometime! Wouldn't that be soooo much fun?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Nephia",
        args![
            "Wow~",
            "Such beautiful hair! I'm sorry, it's just that I love pretty hairstyles, and accessories!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Nephia", args!["Ooh~! You know what would make your hair so much cuter? I think if you tied it back with a big, red ribbon, you would look sooo ^FF66CCadorable^000000!"])?;
    ctx.next()?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Um, lady, I'm a dude.:^FF66CCOoh~! You're right!^000000")],
        )?) == 1
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Woman, can't you that I'm a man?"],
            )?;
            ctx.next()?;
            ctx.lines_as("Nephia", args!["Awww...", "But you would look so ^FF66CCpretty~!^000000"])?;
            ctx.next()?;
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["That...", "Was the worst", "compliment anyone", "has ever given me."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oooh~!", "That sounds great!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nephia",
            args!["Like, oh my God! That's what I was thinking! Oh hey~! I can go ahead and tie that ribbon for you if you want!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nephia",
            args![
                "Ooh! Bring...",
                "1 ^FF0000Big Ribbon^000000,",
                "1 ^FF0000slotted Ribbon^000000 and",
                "1 ^FF0000Silk Ribbon^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nephia", args!["Oh, I almost forgot!", "^FF0000If you bring me any upgraded items, or items compounded with cards, those cards and upgrades will be lost once I create the new item for you. So please don't forget that!^000000"])?;
        ctx.next()?;
        ctx.lines_as("Nephia", args!["Ooh, hurry hurry! I can't wait to accessorize your hair!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("No thanks.:^FF66CCOoh~! You're right!^000000")],
        )?) == 1
        {
            ctx.lines_as(
                ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                args!["Er...", "No thanks."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nephia",
                args![
                    "Awwww~",
                    "But you're just the cutest little thing. I just know you would look so preeeetty!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Oooh~!", "That sounds great!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nephia",
            args!["Like, oh my God! That's what I was thinking! Oh hey~! I can go ahead and tie a ribbon for you if you want!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Nephia",
            args![
                "Ooh! To do that I need",
                "1 ^FF0000Big Ribbon^000000,",
                "1 ^FF0000slotted Ribbon^000000 and",
                "1 ^FF0000Silk Ribbon^000000!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nephia", args!["Oh, I almost forgot!", "^FF0000If you bring me any upgraded items, or items compounded with cards, those cards and upgrades will be lost once I create the new item for you. So please don't forget that!^000000"])?;
        ctx.next()?;
        ctx.lines_as("Nephia", args!["Ooh, hurry hurry! I can't wait to accessorize your hair!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn nephia_1(ctx: &Ctx) -> Script {
    nephia_1_body(ctx, Vec::new()).map(|_| ())
}

fn meruntei_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(5010)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(5049)])?.number()? > 0)
        && ctx.call(Function::CountItem, vec![Val::from(7101)])?.number()? > 9)
        && ctx.var("Zeny").get()?.number()? > 9999)
    {
        ctx.lines_as(
            "Meruntei",
            args!["Oh! You brought all of them! Now I shall make you an Indian Headband of your very own."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Meruntei",
            args![
                "Take this! The items you have brought will be very useful for continuing my work in spreading headbands around Midgard."
            ],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(5010), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(5049), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(7101), Val::from(10)])?;
        ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
        ctx.call(Function::GetItem, vec![Val::from(5071), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as("Meruntei", args!["iiiyiyiyiyiyiyiiiiii~~!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Meruntei",
        args![
            "iiiyiyiyiyiyiyiiiiii~~!",
            " ",
            "Indian spirit, forever~!",
            "On behalf of the Comodo Indians, I give reverence to all Indian Tribes!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Meruntei", args!["Would you like to have an Indian Headband? You wouldn't become an official member of my tribe, but wearing it would show your respect if you happen to encounter tribal Indians in your adventures."])?;
    ctx.next()?;
    ctx.lines_as(
        "Meruntei",
        args![
            "You could just bring...",
            "1 ^FF0000Indian Fillet^000000,",
            "1 ^FF0000Striped Hairband^000000,",
            "10 ^FF0000PecoPeco Feather^000000",
            "and ^FF000010,000 zeny!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Meruntei", args!["I will be using the items you've given me so I can continue my work in spreading Indian Headbands, and paying homage to the tribes of Midgard."])?;
    ctx.next()?;
    ctx.lines_as("Meruntei", args!["^FF0000For your information, if you bring me any item that has a card inserted, or has been upgraded, the additional abilities of the item will disappear after I have used them. So please keep that in mind.^000000"])?;
    ctx.next()?;
    ctx.lines_as("Meruntei", args!["iiiyiyiyiyiyiyiiiiii~~!"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn meruntei_1(ctx: &Ctx) -> Script {
    meruntei_1_body(ctx, Vec::new()).map(|_| ())
}

fn ipore_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::CountItem, vec![Val::from(921)])?.number()? > 299 {
        ctx.lines_as(
            "Ipore",
            args!["Wow! Did you really gather these 300 Mushroom Spores all by yoruself? Nice! Now it's time to use my magic~"],
        )?;
        ctx.next()?;
        ctx.lines(args![
            "^3355FF*Sprinkle Sprinkle*",
            "Ipore pours the Mushroom Spores onto your head.^000000"
        ])?;
        ctx.next()?;
        ctx.lines_as("Ipore", args!["Hocus pocus!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Ipore",
            args![
                ".......................",
                "Phew! Success! There's a mushroom growing out of your head!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ipore",
            args![
                "But...",
                "I don't like the way it's tilting. It should have grown in the ^FF3333other^000000 direction."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ipore",
            args![
                "I guess...",
                "I guess we should make that mushroom into a headband so that you can adjust it to your style."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ipore",
            args![
                "I know, I know...",
                "It's not as cool as having the mushroom actually attached to your skull, but it's all we can do. I'm sorry I let you down."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Ipore",
            args!["...There.", "It's done. But are you okay? Luckily, I don't think I scalped you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Ipore", args!["Anyway, now you can wear that mushroom at a rakish angle, the way it's supposed to be worn. Stay cool, you fashion rebel, you."])?;
        ctx.call(Function::DelItem, vec![Val::from(921), Val::from(300)])?;
        ctx.call(Function::GetItem, vec![Val::from(5082), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Ipore",
        args![
            "Finally...",
            "I've created a magic spell that can make mushrooms grow anywhere! Anyplace you can imagine, I can make a mushroom grow there."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Ipore", args!["Even...", "On top of the human head..."])?;
    ctx.next()?;
    ctx.lines_as("Ipore", args!["Some may think it grotesque, but wouldn't you agree that sporting a mushroom, grown from your head, would start an art revolution?"])?;
    ctx.next()?;
    ctx.lines_as(
        "Ipore",
        args![
            "Just think! It's the perfect fusion of magic and art, fashion and living life...",
            "Man and mushroom?",
            "Ah...I've piqued your curiosity I see."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Ipore", args!["So...", "But in order to cast this spell, I need the magical power in ^FF0000300 Mushroom Spores^000000. So, if you bring me all of those Mushroom Spores, you can see my magic for yourself!"])?;
    ctx.next()?;
    ctx.lines_as("Ipore", args!["Now now, you do understand that I have to make the mushroom grow out of your head. Anywhere else, you could suspect that there's some kind of trick or illusion involved."])?;
    ctx.next()?;
    ctx.lines_as("Ipore", args!["Besides, nothing says 'rebel without a cause' like a mushroom growing out of your head! You would look soooo cool, independent and countercultured!"])?;
    ctx.next()?;
    ctx.lines_as(
        "Ipore",
        args![
            "Remember, bring me...",
            "^FF0000300 Mushroom Spore^000000,",
            "so that I can let you experience my amazing magic for yourself~!"
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn ipore_1(ctx: &Ctx) -> Script {
    ipore_1_body(ctx, Vec::new()).map(|_| ())
}

fn old_blacksmith_hgear_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Skillful Looking Artisan",
        args!["Aha~", "You must really like to travel! You're certainly not from around here."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Skillful Looking Artisan",
        args!["Allow me to introduce myself. My name is Hatbyr Mhore, a travelling Blacksmith."],
    )?;
    ctx.next()?;
    if (ctx.call(Function::CountItem, vec![Val::from(2254)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(2286)])?.number()? > 0)
    {
        ctx.lines_as(
            "Hatbyr Mhore",
            args!["Oh? It seems that you're carrying some valuable stuff with you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Hatbyr Mhore", args!["Hmm, if you like, I could make you something truly amazing with that ^4d4dffAngel Wing^000000 and ^4d4dffElven Ears^000000 that you have."])?;
        ctx.next()?;
        ctx.lines_as("Hatbyr Mhore", args!["How does that sound?", "Oh, and don't you worry about my skill! I'm pretty well known among Blacksmiths for my talent, and my knack of making great things out of junk."])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Umm, let me think.:Okay, make it for me then.:Can you make it with an Evil Wing..?",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["Huh. What's to think about?! I was gonna use all of my skill to create something special for you. Ah well..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["Aha~ Good good good.", "Let's see let's see...", "1 Angel Wing", "1 Elven Ears..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args![
                        "Some other crap and",
                        "...20,000 zeny.",
                        "I must say this is a very cheap price to pay for such a great item. But before I start, I should tell you..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hatbyr Mhore", args!["I don't care how many times you've upgraded the items you brought, but please ^4d4dff carry only items that you need to create this item.^000000 I can't mess up my artwork because of some mistake you might make."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I am ready.:Okay, let me go store my other items first.")],
                )?) == 1
                {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                        ctx.lines_as(
                            "Hatbyr Mhore",
                            args![
                                "Ouch...!",
                                "Why are you carrying so many items with you? Leave all your extra baggage somewhere else and come back."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ((ctx.call(Function::CountItem, vec![Val::from(2254)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(2286)])?.number()? > 0)
                            && ctx.var("Zeny").get()?.number()? > 19999)
                        {
                            ctx.lines_as("Hatbyr Mhore", args!["Alright...", "Let's get a groove on!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args![
                                    "There you go, buddy.",
                                    "I am proud to say this is my masterpiece. Please take this item. I call it...",
                                    "'Angel Wing Ears!'"
                                ],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(2254), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(2286), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(5074), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args!["There's no doubt that you'll be the talk of the town sporting these fashionable things."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args![
                                    "The glamour of an angel and the cuteness of Elven Ears is almost too much goodness for one headgear~!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Hatbyr Mhore", args!["I'm sorry buddy, but I can't make this item without having the things I need. Remember, I need 1 Angel Wing, 1 Elven Ears and 20,000 zeny."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["No problem.", "Come back anytime you want.", "I'll just be here enjoying the view."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args![
                        "Of course, I can make an item",
                        "out of an Evil Wing as well!",
                        "That sort of thing is simple for me!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["Okay then, I will need...?", "1 Evil Wing,", "1 Elven Ears,", "...and 20,000 zeny."],
                )?;
                ctx.next()?;
                ctx.lines_as("Hatbyr Mhore", args!["I must say this is a very cheap price considering the item I will make for you. But before I start, I should tell you..."])?;
                ctx.next()?;
                ctx.lines_as("Hatbyr Mhore", args!["I don't care how many times you've upgraded the items you brought, but please ^4d4dff carry only items that you need to create this item.^000000 I can't mess up my artwork because of some mistake you might make."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I am ready.:Okay, let me go store my other items first.")],
                )?) == 1
                {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                        ctx.lines_as(
                            "Hatbyr Mhore",
                            args![
                                "Ouch...!",
                                "Why are you carrying so many items with you? Leave all your extra baggage somewhere else and come back."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ((ctx.call(Function::CountItem, vec![Val::from(2255)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(2286)])?.number()? > 0)
                            && ctx.var("Zeny").get()?.number()? > 19999)
                        {
                            ctx.lines_as("Hatbyr Mhore", args!["Alright, it's time to roll!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args!["There you go, buddy. I am proud to say this is my masterpiece. I call it... 'Devil Wing Ears!'"],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(2255), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(2286), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(5068), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as("Hatbyr Mhore", args!["You'll be wowing everyone on the streets with your fashionable new look that says 'It feels so good to be so bad.' Glad to be of service~!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Hatbyr Mhore", args!["I'm sorry buddy, but I can't make this item without the stuff I need. Remember I need 1 Devil Wing, 1 Elven Ears and 20,000 zeny."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["No problem.", "Come back anytime you want.", "I'll just be here enjoying the view."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else if (ctx.call(Function::CountItem, vec![Val::from(2255)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(2286)])?.number()? > 0)
    {
        ctx.lines_as(
            "Hatbyr Mhore",
            args!["Oh? It seems that you're carrying some valuable stuff with you."],
        )?;
        ctx.next()?;
        ctx.lines_as("Hatbyr Mhore", args!["Hmm, if you like, I could make you something truly amazing with that ^4d4dffEvil Wing^000000 and ^4d4dffElven Ears^000000 that you have."])?;
        ctx.next()?;
        ctx.lines_as("Hatbyr Mhore", args!["How does that sound?", "Oh, and don't you worry about my skill! I'm pretty well known among Blacksmiths for my talent, and my knack of making great things out of junk."])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from(
                "Umm, let me think.:Okay, make it for me then.:Can you make it with an Angel Wing..?",
            )],
        )? {
            1 => {
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["Huh. What's to think about?! I was gonna use all of my skill to create something special for you. Ah well..."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["Aha~ Good good good.", "Let's see let's see...", "1 Devil Wing", "1 Elven Ears..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args![
                        "Some other crap and",
                        "...20,000 zeny.",
                        "I must say this is a very cheap price to pay for such a great item. But before I start, I should tell you..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hatbyr Mhore", args!["I don't care how many times you've upgraded the items you brought, but please ^4d4dff carry only items that you need to create this item. I can't mess up my artwork because of some mistake you might make. ^000000"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I am ready.:Okay, let me go store my other items first.")],
                )?) == 1
                {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                        ctx.lines_as(
                            "Hatbyr Mhore",
                            args![
                                "Ouch, why are you carrying",
                                "so many items with you?",
                                "Leave all your extra baggage somewhere else and come back."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ((ctx.call(Function::CountItem, vec![Val::from(2255)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(2286)])?.number()? > 0)
                            && ctx.var("Zeny").get()?.number()? > 19999)
                        {
                            ctx.lines_as("Hatbyr Mhore", args!["Alright...", "Let's get a groove on!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args![
                                    "There you go, buddy.",
                                    "I am proud to say this is my masterpiece. Please take this item. I call it... 'Devil Wing Ears!'"
                                ],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(2255), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(2286), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(5068), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as("Hatbyr Mhore", args!["You'll be wowing everyone on the streets with your fashionable new look that says 'It feels so good to be so bad.' Glad to be of service~!"])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Hatbyr Mhore", args!["I'm sorry buddy, but I can't make this item without having the things I need. Remember, I need 1 Devil Wing, 1 Elven Ears and 20,000 zeny."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args!["No problem.", "Come back anytime you want.", "I'll just be here enjoying the view."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args![
                        "Of course, I can make an item",
                        "out of an Angel Wing as well!",
                        "That sort of thing is simple for me!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args![
                        "Okay then, then I'll need...",
                        "1 Angel Wing,",
                        "1 Elven Ears,",
                        "...and 20,000 zeny."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Hatbyr Mhore", args!["I must say this is a very cheap price considering the item I will make for you. But before I start, I should tell you..."])?;
                ctx.next()?;
                ctx.lines_as("Hatbyr Mhore", args!["I don't care how many times you've upgraded the items you brought, but please ^4d4dff carry only items that you need to create this item.^000000 I can't mess up my artwork because of some mistake you might make."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("I am ready.:Okay, let me go store my other items first.")],
                )?) == 1
                {
                    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
                        ctx.lines_as(
                            "Hatbyr Mhore",
                            args![
                                "Ouch, why are you carrying",
                                "so many items with you?",
                                "Leave all your extra baggage somewhere else and come back."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ((ctx.call(Function::CountItem, vec![Val::from(2254)])?.number()? > 0
                            && ctx.call(Function::CountItem, vec![Val::from(2286)])?.number()? > 0)
                            && ctx.var("Zeny").get()?.number()? > 19999)
                        {
                            ctx.lines(args!["Alright...", "Let's get a groove on!"])?;
                            ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_REPAIRWEAPON")?])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args![
                                    "There you go, buddy.",
                                    "I am proud to say this is my masterpiece. Please take this item. I call it... 'Angel Wing Ears!'"
                                ],
                            )?;
                            ctx.call(Function::DelItem, vec![Val::from(2254), Val::from(1)])?;
                            ctx.call(Function::DelItem, vec![Val::from(2286), Val::from(1)])?;
                            ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(20000))?))?;
                            ctx.call(Function::GetItem, vec![Val::from(5074), Val::from(1)])?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args!["There's no doubt that you'll be the talk of the town sporting these fashionable things."],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Hatbyr Mhore",
                                args![
                                    "The glamour of an angel and the cuteness of Elven Ears is almost too much goodness for one headgear~!"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as("Hatbyr Mhore", args!["I'm sorry buddy, but I can't make this item without having the things I need. Remember, I need 1 Angel Wing, 1 Elven Ears and 20,000 zeny."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                ctx.lines_as(
                    "Hatbyr Mhore",
                    args![
                        "No problem.",
                        "Come back anytime you want.",
                        " ",
                        "I'll just be here enjoying the view."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Hatbyr Mhore", args!["I happened to come to Juno while I was traveling around the world. Being someplace up in the clouds makes me a little nervous, but still, the scenery looks great from up here."])?;
        ctx.next()?;
        ctx.lines_as("Hatbyr Mhore", args!["Although I really enjoy traveling more than staying in my shop, I'm still a Blacksmith at heart and I always gotta be making something. It's been a while since I've made my last creation..."])?;
        ctx.next()?;
        ctx.lines_as(
            "Hatbyr Mhore",
            args![
                "Hmm...",
                "If I had some materials, I think I could put together some pretty amazing stuff."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Hatbyr Mhore",
            args![
                "If by any chance, you have an Angel Wing or Evil Wing, and",
                "Elven Ears, would you give me a chance to smith something?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Hatbyr Mhore", args!["I was thinking...", "Elven Ears are great...", "And everyone loves Angel Wing or Devil Wing. Wouldn't it be totally fab if I could combine the Elven Ears with one of the wings?!"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from("Give me more information.:Yeah, I will think about it.")],
        )?) == 1
        {
            ctx.lines_as("Hatbyr Mhore", args!["Oh, right.", "Let me tell", "you exactly I need."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hatbyr Mhore",
                args![
                    "For creating Angel Wing Ears, I need 1 ^4d4dffAngel Wing^000000,",
                    "1 ^4d4dffElven Ears^000000 and",
                    "^4d4dff20,000 zeny^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hatbyr Mhore",
                args![
                    "For creating Devil Wing Ears,",
                    "I need 1 ^4d4dffDevil Wing^000000,",
                    "1 ^4d4dffElven Ears^000000 and",
                    "^4d4dff20,000 zeny^000000."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hatbyr Mhore",
                args![
                    "Come back anytime when you have those materials.",
                    "I will let you have either the elegance of an angel or the charisma of a demon with the wearables only I can craft."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Hatbyr Mhore",
            args![
                "No problem.",
                "I'm always here at this scenic spot so I can enjoy the view. Just come back whenever you feel like it."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn old_blacksmith_hgear(ctx: &Ctx) -> Script {
    old_blacksmith_hgear_body(ctx, Vec::new()).map(|_| ())
}

fn pretty_lindsay_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
        ctx.lines_as("Pretty Lindsay", args!["I know you brought everything you need for me to make you a hat, but you're carrying too much stuff. Why don't you put some of your things in Kafra Storage?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if (ctx.call(Function::CountItem, vec![Val::from(5033)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(5064)])?.number()? > 0)
    {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
        ctx.lines_as(
            "Pretty Lindsay",
            args!["Whoa~", "You brought everything!", "Okay, hold on a bit.", "Let me make your hat."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pretty Lindsay",
            args!["As my mom taught me,", "put this down on the ground...", "Okay, I am ready..."],
        )?;
        ctx.next()?;
        ctx.mes("^FF0000Lindsay put the Raccoon Hat on the ground, threw the Smokie Leaf onto the hat and mumbled some words while hugging herself.^000000")?;
        ctx.next()?;
        ctx.mes(
            "^FF0000Then, suddenly with a dazzling light, the Raccoon Hat turned blue and slowly transformed into a Sea-Otter Hat.^000000",
        )?;
        ctx.call(Function::NpcSpecialEffect, vec![ctx.constant("EF_FLASHER")?])?;
        ctx.next()?;
        ctx.lines_as(
            "Pretty Lindsay",
            args!["Phew! It's done!", "Gosh! I think I", "spent all my energy.", "Okay, take this~"],
        )?;
        ctx.call(Function::DelItem, vec![Val::from(5033), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(5064), Val::from(1)])?;
        ctx.call(Function::GetItem, vec![Val::from(5078), Val::from(1)])?;
        ctx.next()?;
        ctx.lines_as(
            "Pretty Lindsay",
            args!["I made this hat with all my heart, so you gotta promise me you will take care of this, okay?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.lines_as("Pretty Lindsay", args!["Hello, there?", "Mister...?"])?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.lines_as(
            "Pretty Lindsay",
            args![
                "You look like you're freezing...",
                "Um, I know! How about I make you a cute, warm and fuzzy Sea-Otter hat?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pretty Lindsay",
            args!["It's really really cute! Also, it's the only hat I know how to make, because I love playing with Sea Otters so much!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Pretty Lindsay", args!["I have to put together ^0000FF1 Raccoon Hat^000000 and ^0000FF1 Smokie Leaf^000000 to make a ^4D4DFFSea-Otter Hat^000000 for you, though."])?;
        ctx.next()?;
        if Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Okay, I will come back with the stuff.:Um... How come you need a Raccoon Hat?",
            )],
        )?) == 1
        {
            ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
            ctx.lines_as("Pretty Lindsay", args!["Cool~! Come back soon as you can~!"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Pretty Lindsay",
            args!["Well, it's supposed to be a seeeecret, but my mommy taught me some magic to make Sea-Otter hats with Raccoon hats."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pretty Lindsay",
            args!["It's the bestest magic ever because it comes from my heart!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["That...", "That still doesn't explain anything!"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
            args!["Alright...", "Yeah...", "I'll come back later, kid."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Pretty Lindsay", args!["Hello, hello~!", "You're such a pretty lady!"])?;
    ctx.next()?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
    ctx.lines_as(
        "Pretty Lindsay",
        args!["Hey hey! Can I make you a Sea-Otter hat! It's really really really really cute!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pretty Lindsay",
        args!["I need ^0000FF1 Raccoon Hat^000000 and ^0000FF1 Smokie Leaf^000000 so I can make a ^4D4DFFSea-Otter Hat^000000 for you."],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from(
            "Okay, I will come back with the stuff.:Um, why do you want to make hats so much?",
        )],
    )?) == 1
    {
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.lines_as("Pretty Lindsay", args!["Cool~!", "Come back as", "soon as you can~!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Pretty Lindsay",
        args![
            "Because...!",
            "If everyone in Lutie was wearing a fuzzy Sea-Otter cap, it would look so so cute!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pretty Lindsay",
        args![
            "We could play games in the snow, and play with real Sea Otters and have them give us rides and...",
            "....."
        ],
    )?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn pretty_lindsay_1(ctx: &Ctx) -> Script {
    pretty_lindsay_1_body(ctx, Vec::new()).map(|_| ())
}

fn fuzzy_fuzz_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (((ctx.call(Function::CountItem, vec![Val::from(5030)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(7213)])?.number()? > 99)
        && ctx.call(Function::CountItem, vec![Val::from(7217)])?.number()? > 99)
        && ctx.call(Function::CountItem, vec![Val::from(7161)])?.number()? > 299)
    {
        if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2000 {
            ctx.lines_as("Fuzzy Fuzz", args!["You brought every material I need, but unfortunately you don't have enough space in your inventory. Try and place some of your items into Kafra Storage first."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Fuzzy Fuzz",
            args!["Excellent! You brought everything! Please make yourself comfortable while I make you your Teddybear Hat."],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
        ctx.lines_as(
            "Fuzzy Fuzz",
            args!["Hmm Hm Hmm...", "Tear this off...", "Knit this on....", "Okay, it's almost done..."],
        )?;
        ctx.next()?;
        ctx.mes("[Fuzzy Fuzz]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_FRET")?])?;
        ctx.mes(
            "There is an old saying, 'Everything comes to those who wait.' Oh, are you freezing? Why don't you rub your hands together?",
        )?;
        ctx.next()?;
        ctx.mes("[Fuzzy Fuzz]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
        ctx.mes("It's complete! There you go. I think this one will look really good on you! Please take good care of this hat.")?;
        ctx.call(Function::DelItem, vec![Val::from(5030), Val::from(1)])?;
        ctx.call(Function::DelItem, vec![Val::from(7213), Val::from(100)])?;
        ctx.call(Function::DelItem, vec![Val::from(7217), Val::from(100)])?;
        ctx.call(Function::DelItem, vec![Val::from(7161), Val::from(300)])?;
        ctx.call(Function::GetItem, vec![Val::from(5059), Val::from(1)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    ctx.lines_as(
        "Fuzzy Fuzz",
        args![
            "Hello there, young man.",
            "What brings you to Lutie, town of goodwill and year round Christmas cheer?"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Fuzzy Fuzz", args!["I'm a master of creating stuffed animals, which make great presents. As we all know, everyone loves to hug cute and cuddly stuffed animals."])?;
    ctx.next()?;
    ctx.lines_as("Fuzzy Fuzz", args!["My specialty is the 'Teddybear Hat.' When it's worn, it makes you as cute as a teddy bear, and your hugs twice as warm and cuddly."])?;
    ctx.next()?;
    ctx.mes("[Fuzzy Fuzz]")?;
    if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
        ctx.mes("The 'Teddybear Hat' can't be worn by just anyone, since it's cuddling powers are easily abused.")?;
        ctx.next()?;
        ctx.lines_as(
            "Fuzzy Fuzz",
            args!["But I sense that you are an adventurer with a pure heart and would not use the Teddybear Hat for selfish gain."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Fuzzy Fuzz",
            args!["You can learn a lot about telling good people from the naughty after working for Santa for a loooong time..."],
        )?;
    } else {
        ctx.mes("I sense that you are an adventurer with a good heart, and that a sweet girl like you deserves a nice gift.")?;
        ctx.next()?;
        ctx.lines_as(
            "Fuzzy Fuzz",
            args!["You know more about such things after working for Santa for so long."],
        )?;
    }
    ctx.next()?;
    ctx.lines_as("Fuzzy Fuzz", args!["Unfortunately, I don't have any material to make the headgear right now. But if you bring the materials, I will make you a 'Teddybear Hat' for free."])?;
    ctx.next()?;
    ctx.lines_as(
        "Fuzzy Fuzz",
        args![
            "Let's see...",
            "I'll need...",
            "^0000FF1 Panda Hat^000000,",
            "^0000FF100 Needle Packet^000000,",
            "^0000FF100 Spool^000000 and",
            "^0000FF300 Black Bear Skin^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Fuzzy Fuzz",
        args!["So, do you think you can get all of them? I will be here waiting for you."],
    )?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn fuzzy_fuzz_1(ctx: &Ctx) -> Script {
    fuzzy_fuzz_1_body(ctx, Vec::new()).map(|_| ())
}

fn nanhyang_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            "- Wait a minute! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again after -",
            "- you put some items into Kafra Storage. -"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.var("MaxWeight").get()?.try_sub(ctx.var("Weight").get()?)?).number()? < 2430 {
        ctx.lines_as("Nanhyang", args!["Hmmm...", "You look like you're having trouble carrying all of your things. Why don't you place some of your extra items into the Kafra Storage?"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(5073)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(1750)])?.number()? > 0)
    {
        ctx.lines_as(
            "Nanhyang",
            args![
                "Hello, there.",
                "Ah, I can tell that you already have a ^8C1717'Model Training Hat'^000000. I see that using it has corrected your posture."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nanhyang", args!["Oh...", "I forgot to tell you that I can not only put together a 'Model Training Hat,' I can disassemble it into its original parts."])?;
        ctx.next()?;
        ctx.lines_as("Nanhyang", args!["If you give me ^0000FF1 'Model Training Hat'^000000 and ^0000FF1 'Arrow'^000000, I give you ^8C17171 'Apple of Archer'^000000 and ^8C17171 'Book'^000000. So, would you like to disassemble the items?"])?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Disassemble:Cancel")])?) == 1 {
            ctx.lines_as(
                "Nanhyang",
                args![
                    "Thank you.",
                    "It will not take that much time to make this hat, so please wait a moment."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nanhyang",
                args![
                    " . . . . . . . . . . . . . .",
                    " . . . . . . . . . . . . . ",
                    " . . . . . . . . . . . . . .",
                    " . . . . . . . . . . . . . "
                ],
            )?;
            ctx.next()?;
            ctx.mes("[Nanhyang]")?;
            ctx.call(Function::DelItem, vec![Val::from(5073), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(1750), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(2285), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(1550), Val::from(1)])?;
            ctx.mes("There you go.")?;
            ctx.next()?;
            ctx.lines_as(
                "Nanhyang",
                args!["If you want to make a", "'Model Training Hat' again,", "feel free to come back."],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.lines_as(
            "Nanhyang",
            args![
                "I see.",
                "I hope you will",
                "continue to use that hat",
                "to improve your posture",
                "and glamorous poise."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if (ctx.call(Function::CountItem, vec![Val::from(2285)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(1550)])?.number()? > 0)
    {
        ctx.mes("[Nanhyang]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
        ctx.mes("You brought the materials~")?;
        ctx.next()?;
        ctx.lines_as(
            "Nanhyang",
            args!["Before I start to create the item, I'm obligated to warn you..."],
        )?;
        ctx.next()?;
        ctx.lines_as("Nanhyang", args!["If the ^FF0000'Book' and 'Apple of Archer' items you have brought to make a 'Model Training Hat' was ^FF0000upgraded^000000, after I create the hat out of the book, ^FF0000these upgrades, including compounded cards, will be lost.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Hanhyang",
            args!["So would you like to create the Model Training Hat right now?"],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("Create:Cancel")])?) == 1 {
            ctx.mes("[Nanhyang]")?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_OK")?])?;
            ctx.lines(args![
                "Thank you.",
                "Let me create one for you right away. Please have a seat and make yourself comfortable."
            ])?;
            ctx.next()?;
            ctx.lines_as(
                "Nanhyang",
                args!["Usually we have a zeny service charge, but since you've got ^3355FFthe look^000000, I'll waive it."],
            )?;
            ctx.next()?;
            ctx.lines_as("Nanhyang", args!["Please wait a moment while I put this together."])?;
            ctx.next()?;
            ctx.lines(args![
                " . . . . . . . . . . . . . .",
                " . . . . . . . . . . . . . ",
                " . . . . . . . . . . . . . .",
                " . . . . . . . . . . . . . "
            ])?;
            ctx.next()?;
            ctx.mes("[Nanhyang]")?;
            ctx.call(Function::Emotion, vec![ctx.constant("ET_AHA")?])?;
            ctx.mes("That wasn't too difficult. I hope you wear this hat everyday to improve your posture.")?;
            ctx.next()?;
            ctx.lines_as("Nanhyang", args!["You may have trouble at first, keeping your balance and trying not to drop the apple. You may even feel incredible neck pain. But for the sake of beauty, it'll all be worth it."])?;
            ctx.next()?;
            ctx.lines_as(
                "Nanhyang",
                args![
                    "Here it is.",
                    "Oh, right, this arrow from the Apple of Archer is left over, so you can go ahead and take that."
                ],
            )?;
            ctx.call(Function::DelItem, vec![Val::from(2285), Val::from(1)])?;
            ctx.call(Function::DelItem, vec![Val::from(1550), Val::from(1)])?;
            ctx.call(Function::GetItem, vec![Val::from(5073), Val::from(1)])?;
            ctx.next()?;
            ctx.lines_as("Nanhyang", args!["Also, I can disassemble the Model Training Hat into its original materials. So if you want the original items back, bring the Model Training Hat and the arrow I gave you."])?;
            ctx.call(Function::GetItem, vec![Val::from(1750), Val::from(1)])?;
            ctx.next()?;
            ctx.lines_as("Nanhyang", args!["I hope you", "will have a", "good day.", "Farewell."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.mes("[Nanhyang]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_THINK")?])?;
        ctx.mes("I see. You must have brought items with cards or upgrades that you didn't want to lose. Feel free to come back anytime when you are ready.")?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.mes("[Nanhyang]")?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SURPRISE")?])?;
    ctx.lines(args![
        "Hello~",
        "Welcome to the Handsome Charm Modeling School. May I help you?"
    ])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("I want to be a model~!:Cancel")])?) == 1 {
        ctx.lines_as(
            "Nanhyang",
            args![
                "I see~",
                "Although it's clear that you've got '^3355FFthe look^000000,' we can't accept any students at the moment."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nanhyang", args!["Lately, our director has been gone, and we aren't sure when he'll come back. ^333333*Sigh* I can't even remember the last time we got paid.^000000"])?;
        ctx.next()?;
        ctx.lines_as(
            "Nanhyang",
            args![
                "Hmm... Wait a minute!",
                "At the very least, we could train you to improve your posture so that you can walk with regal poise."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Nanhyang", args!["Although we can't offer any classes at this time, I can create a special 'Model Training Hat' for you. That way, you can practice the glamourous way of walking down the catwalk on your own."])?;
        ctx.next()?;
        ctx.mes("[Nanhyang]")?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_QUESTION")?])?;
        ctx.mes("But since our director is gone, we can't really use our budget to give you a hat. If you want, please bring me the following items...")?;
        ctx.next()?;
        ctx.lines_as(
            "Nanhyang",
            args![
                "Please bring...",
                "^0000FF1 Apple of Archer^000000 and",
                "^0000FF1 Book^000000.",
                "Then I can fashion a 'Model Training Hat' for you right away."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as("Nanhyang", args!["I understand.", "Well, have a good day.", "Thank you~"])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn nanhyang_1(ctx: &Ctx) -> Script {
    nanhyang_1_body(ctx, Vec::new()).map(|_| ())
}

fn seth_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CheckWeight, vec![Val::from(1201), Val::from(1)])? == 0 {
        ctx.lines(args![
            " [Seth]",
            "Whoa, why are you carrying so many items with you?",
            "You look so heavy...! Maybe you can walk more easily if you put some of your stuff away?"
        ])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines(args![
        " [Seth]",
        "Hello, my name is Seth!",
        "I like folding paper and making origami. I made my mommy a big paper boat, and she gave me a big smile and patted my head!"
    ])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
    ctx.next()?;
    ctx.lines_as("Seth", args!["Today at school, I learned how to make a paper flower! The flower was sort of hard, but now I can do it! Then I made a tiny tiny flower and put in on my head~"])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
    ctx.next()?;
    if (ctx.call(Function::CountItem, vec![Val::from(2278)])?.number()? > 0
        && ctx.call(Function::CountItem, vec![Val::from(975)])?.number()? > 0)
    {
        ctx.lines_as(
            "Seth",
            args![
                "Hey~!",
                "You have a Mr. Smile and Scarlet Dyestuffs! Yaaay~! I could use those and make you a paper flower. C'mon, lemme show you!"
            ],
        )?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.next()?;
        ctx.lines_as("Seth", args!["Lemme make", "one for you,", "pwease pwease?"])?;
        ctx.call(Function::Emotion, vec![ctx.constant("ET_PROFUSELY_SWEAT")?])?;
        ctx.next()?;
        match runtime::select_values(
            ctx,
            &[Val::from("Alright!:No thanks kid.:Boys aren't supposed to make flowers.")],
        )? {
            1 => {
                ctx.call(Function::Emotion, vec![ctx.constant("ET_SMILE")?])?;
                ctx.lines_as("Seth", args!["Yay~", "You're the best!"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Seth",
                    args![
                        "Let's see...",
                        "I cut Mr. Smile this way~.",
                        "*Snip Snip Snip~~*",
                        "*Snip...Snip...*",
                        "Yyyyyaaaeeep..."
                    ],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_DELIGHT")?])?;
                ctx.next()?;
                ctx.lines_as("Seth", args!["Tah dah~!"])?;
                ctx.next()?;
                ctx.call(Function::DelItem, vec![Val::from(2278), Val::from(1)])?;
                ctx.call(Function::DelItem, vec![Val::from(975), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(5077), Val::from(1)])?;
                ctx.lines_as("Seth", args!["Heheheh~!", "I'm done!", "I did a good job, didn't I?"])?;
                ctx.next()?;
                if (ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_NOVICE")?)
                    || ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_SUPER_NOVICE")?))
                {
                    ctx.lines_as(
                        "Seth",
                        args!["Oh, here's the leftovers. Mommy says that we should always save things. Heh heh~"],
                    )?;
                    ctx.call(Function::GetItem, vec![Val::from(935), Val::from(1)])?;
                    ctx.next()?;
                }
                ctx.lines_as(
                    "Seth",
                    args!["I'm a good boy!", "I'm gonna make one for my little sister~! And everybody I love!"],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Seth",
                    args!["If you want to see me make another one, just bring more Scarlet Dyestuffs and Mr.Smile, okay? Bye bye~!"],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            2 => {
                ctx.lines_as(
                    "Seth",
                    args!["*Sniff sniff*", "O-okay...", "But I just wanted to show you how good I am..."],
                )?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            3 => {
                ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hey."])?;
                ctx.next()?;
                ctx.lines_as(
                    ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                    args!["Boys aren't supposed to make flowers."],
                )?;
                ctx.next()?;
                ctx.lines_as("Seth", args!["Waaaaaaah~!", "But flowers are so nice and pweety!!!"])?;
                ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    } else {
        ctx.lines_as("Seth", args!["You want me to show you? Huh?", "I can do it you want!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Seth",
            args!["Uh oh...", "But I don't have anything to make paper flowers out of."],
        )?;
        ctx.next()?;
        if Val::from(runtime::select_values(ctx, &[Val::from("I'll get what you need.:Too bad!")])?) == 1 {
            ctx.lines_as(
                "Seth",
                args![
                    "Okay okay...",
                    "I neeeeed...",
                    "^3131FF1 Mr. Smile^000000 and",
                    "^3131FF1 Scarlet Dyestuffs^000000.",
                    "Then I can make you a tiiiiny little flower."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Seth", args!["Seth will wait for you here. Promise me you'll come back?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        ctx.call(Function::Emotion, vec![ctx.constant("ET_CRY")?])?;
        ctx.lines_as("Seth", args!["Wah~~~~~!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn seth_1(ctx: &Ctx) -> Script {
    seth_1_body(ctx, Vec::new()).map(|_| ())
}
