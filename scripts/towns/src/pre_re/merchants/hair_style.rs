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

#[derive(Clone, Copy, Debug)]
enum HairDresserStep {
    Start,
    LCutin,
}

fn hair_dresser_run(ctx: &Ctx, mut step: HairDresserStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_num = Val::from(0);
    let mut l_pallete = Val::from(0);
    let mut l_style = Val::from(0);
    'machine: loop {
        match step {
            HairDresserStep::Start => {
                ctx.lines_as("Veronica", args!["Welcome to Veronica's hair salon.", "How can I help you?"])?;
                ctx.next()?;
                'b1: {
                    let subject1 = Val::from(runtime::select_values(
                        ctx,
                        &[Val::from("See available hair styles.:Change hair style.:End conversation.")],
                    )?);
                    let mut matched1 = false;
                    let no_case1 = !subject1.loosely_equals(&Val::from(1))
                        && !subject1.loosely_equals(&Val::from(2))
                        && !subject1.loosely_equals(&Val::from(3));
                    if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Veronica",
                            args![
                                "We have a total of 19 styles, available from no.1 to no.19.",
                                "Which one do you want to see?",
                                "If you wish to cancel,",
                                "please enter 0."
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_style = input;
                        if l_style.clone().number()? > 19 {
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "Oops, I'm sorry, but that",
                                    "style is not available.",
                                    "Remember to enter a number",
                                    "from 1 to 19."
                                ],
                            )?;
                            ctx.next()?;
                        } else if l_style.clone() == 0 {
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "So, how do you like the style?",
                                    "Feel free to ask me about any",
                                    "available hairstyle. It will",
                                    "be my pleasure to style your",
                                    "hair."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else {
                            hair_dresser_run(ctx, HairDresserStep::LCutin, vec![l_style.clone()])?;
                            ctx.mes("[Veronica]")?;
                            match l_style.number()? {
                                1 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Play Dead' style!",
                                            "It's a nice, basic haircut.",
                                            "I notice that usually the",
                                            "cute, conversative types seem",
                                            "to prefer this style."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'First Aid' style!",
                                        "The shoulder length tresses",
                                        "are straightened for those",
                                        "no nonsense adventurers. It",
                                        "seems to be the style of",
                                        "choice for Novices."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                2 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's the 'Two Handed Sword",
                                            "Mastery' style! It's perfect for",
                                            "for Swordmen who might muss their",
                                            "hair while swinging their swords",
                                            "all day long."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Bash' style!",
                                        "For the powerful woman that's",
                                        "not afraid to get a little",
                                        "blood on her hands, but knows",
                                        "how great her hair will look",
                                        "while wildly flailing a sword."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                3 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Napalm Beat' style!",
                                            "It's a unique look with a hint",
                                            "of eccentricity that's offset",
                                            "with a helping of elegance."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Frost Diver' style!",
                                        "The pigtails lend an innocent,",
                                        "demure look for those Mages",
                                        "and Wizards that usually scare",
                                        "off the boys with their spells."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                4 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's the 'Double Strafe'",
                                            "style! The arrangement of the",
                                            "hair conducts ambient static",
                                            "electricity, naturally clearing",
                                            "the mind. At least, that's what",
                                            "I was taught in fashion school."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Arrow Shower' style!",
                                        "For the Bowswoman who doesn't",
                                        "want fashion to interfere with",
                                        "her depth perception. Much more",
                                        "attractive than those horrid",
                                        "granny-style hairbuns."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                5 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Angelus' style!",
                                            "It's for calm and devout people,",
                                            "as well as those bashful,",
                                            "mild-mannered types."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Heal' style!",
                                        "This is in trend among",
                                        "Priests and Acolytes since",
                                        "this style is appropriate",
                                        "for formal situations, but",
                                        "is also practical in battle."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                6 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Push Cart' style!",
                                            "It was based on the design of a",
                                            "cart...at least, that's what",
                                            "I learned in beautician school."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Ooh, that's 'Vending' style!",
                                        "It's the hairdo of money",
                                        "makers...and if I may say so,",
                                        "it's also economical."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                7 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Ooh, that's 'Envenom' style!",
                                            "It looks great on Thieves and",
                                            "and Assassins when they're",
                                            "out poisoning people and animals.",
                                            "It's fashion for the aggressive",
                                            "and eclectic~!"
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Double Attack' style!",
                                        "The adorable pigtail, paired with",
                                        "those provacative bangs are sure",
                                        "to help you steal the heart of",
                                        "some cute guy."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                8 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Bowling Bash' style!",
                                            "A popular style for Knights, its",
                                            "manly, rugged look tends to",
                                            "attract all of the ladies,",
                                            "and looks great on men with",
                                            "strong chins."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Gloria' style!",
                                        "It's very elegant and looks",
                                        "great on holy Priests. This",
                                        "style is most attractive to",
                                        "ladies who aren't that used",
                                        "to fighting with their hands."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                9 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Venom Dust' style!",
                                            "Definitely a look for rebels,",
                                            "the sweeping, yet decidedly",
                                            "luxorious locks seems to enchant",
                                            "girls with a fatal attraction."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'SP Recovery' style!",
                                        "To add more body to the special",
                                        "style of these bangs, I use a",
                                        "special conditioner that makes",
                                        "you feel like you're regaining SP",
                                        "...Although, it acutally doesn't."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                10 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Turn Undead' style!",
                                            "This is popular among Priests",
                                            "that want a serious, yet a bit",
                                            "of a wild, agressive look.",
                                            "Definitely more attractive",
                                            "than the 'Holy Light' mullet."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Prepare Potion' style!",
                                        "The flared out tresses are chosen",
                                        "by beginning Alchemists, since",
                                        "early, explosive experiments would",
                                        "make their hair to stick out anyway."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                11 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Dragonology' style!",
                                            "It's neat and clean cut, perfect",
                                            "for studious people and looks",
                                            "great with eyeglasses. This",
                                            "is a fashion well suited to",
                                            "intellectual types."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Grand Cross' style!",
                                        "It's in style among those pious",
                                        "Crusaders that need hair that",
                                        "won't muss during fighting, yet",
                                        "is respectable enough to attend",
                                        "religious services."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                12 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Mace Mastery' style!",
                                            "A lot of care goes into making",
                                            "that tussled hair say, 'I don't",
                                            "care how I look at all.'"
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Intimidate' style!",
                                        "The Rogue women seem to like",
                                        "this style...although I imagine",
                                        "that more of them would prefer",
                                        "something wilder to match those",
                                        "stockings..."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                13 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Thunder Storm' style!",
                                            "This hot, flamboyant hairstyle",
                                            "flares out wildly like thunder.",
                                            "...And you will too with this new look."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Spiritual Sphere",
                                        "Absorption' style! There's a",
                                        "charismatic quality to this",
                                        "fashion: it's tough, slightly",
                                        "tomboyish, but not so much",
                                        "that it can't be cute."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                14 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Encore' style!",
                                            "The elegant, flowing locks",
                                            "fit well with Bards, or men who",
                                            "appreciate the value of male",
                                            "beauty."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Gypsy's Kiss' style!",
                                        "Dancers seem to like this style,",
                                        "although personally, I think",
                                        "this fashion fits very well",
                                        "with glasses."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                15 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Grimtooth' style!",
                                            "Spiky and unkempt, this style",
                                            "is a popular counterculture",
                                            "street fashion. You might",
                                            "not want to wear your hair",
                                            "this way at a wedding, though."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Counter Attack' style!",
                                        "This is an intimidating look for",
                                        "girls that want to say 'You hit",
                                        "me, I'll hit you back!' It really",
                                        "emphasizes strong looking",
                                        "foreheads and cheekbones."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                16 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Blitz Beat' style!",
                                            "A funky and lively fashion,",
                                            "this style was developed for a",
                                            "Hunter who liked really long",
                                            "bangs and wanted to see",
                                            "through them at the same time."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Anke Snare' style!",
                                        "The style style is specially",
                                        "made for Hunters that don't like",
                                        "to get their hair tangled...",
                                        "After all, what kind of Hunter",
                                        "lets their hair get trapped?"
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                17 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Find Ore' style!",
                                            "It's a practical, economical look",
                                            "that is popular among Blacksmiths.",
                                            "Some swear that this fashion helps",
                                            "them in finding ores, but where's",
                                            "the science in that??"
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Hammer Fall' style!",
                                        "For the woman that doesn't want",
                                        "her hair to get in the way when",
                                        "she's savagely swinging heavy",
                                        "objects. Of course, this is a",
                                        "Blacksmith favorite."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                18 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Fire Pillar' style!",
                                            "It's a trendy look, in which",
                                            "you cover one eye for that",
                                            "intrigue effect. The element",
                                            "of mystery is always in",
                                            "fashion, don't you think?"
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Jupitel Thunder'",
                                        "style! A look that strikes",
                                        "like lightening, without",
                                        "any of that annoying static",
                                        "cling or muss. This fashion",
                                        "looks great with Mage Hats."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                19 => {
                                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                                        ctx.lines(args![
                                            "Oh, that's 'Guillotine Fist'",
                                            "style! The smooth, slicked back",
                                            "pompadour shows that you're",
                                            "serious about your passion",
                                            "for brawling... or just your passion."
                                        ])?;
                                        ctx.close_window()?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    ctx.lines(args![
                                        "Oh, that's 'Whirlwind' style!",
                                        "A favorite among the studious",
                                        "Sages, the hair is tied back",
                                        "in a stylish braid so that",
                                        "it doesn't fly around after",
                                        "casting those windy spells."
                                    ])?;
                                    ctx.close_window()?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                _ => {}
                            }
                        }
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                        matched1 = true;
                    }
                    if matched1 {
                        if ctx.player().base_level()? < 60 {
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "Oh, dear, you're looking fabulous with",
                                    "your current hairstyle. Why don't you",
                                    "try a new hair accessory rather than changing your look?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if (((((((ctx.items().count(973)? < 3 || ctx.items().count(974)? < 3) || ctx.items().count(901)? < 100)
                            || ctx.items().count(1094)? < 100)
                            || ctx.items().count(1020)? < 100)
                            || ctx.items().count(1060)? < 100)
                            || ctx.items().count(7152)? < 100)
                            || ctx.player().zeny()? < 99800)
                        {
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "If you wish to change your",
                                    "hairstyle, you should meet some",
                                    "requirements. I suggest that you",
                                    "write down all the items that",
                                    "you will need."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "3 Counteragent,",
                                    "3 Mixture,",
                                    "100 Danggie,",
                                    "100 Short Danggie,",
                                    "100 Black Hair,",
                                    "100 Golden Hair,",
                                    "100 Glossy Hair, and lastly..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "You will need 99,800 zeny.",
                                    "Please come back when you're",
                                    "ready. I will make you look",
                                    "fabulous. Hohohohoho~"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Veronica",
                            args![
                                "Okay now, please choose the style",
                                "you desire from styles no.1 to",
                                "no.19.  I will do my best to",
                                "make you look your very best."
                            ],
                        )?;
                        ctx.next()?;
                        let (input, status) = runtime::input_number(ctx, None, None)?;
                        l_style = input;
                        if l_style.clone().number()? > 19 {
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "I am sorry, you chose an unavailable style.",
                                    "Make sure you enter the correct number."
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if l_style.clone() == 0 {
                            ctx.lines_as("Veronica", args!["You have canceled your request."])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        } else if ctx.call(Function::GetLook, args![1])?.loosely_equals(&l_style.clone()) {
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "I am sorry, but you are already",
                                    "wearing the style you have",
                                    "requested. Would you please",
                                    "choose a different style?"
                                ],
                            )?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        hair_dresser_run(ctx, HairDresserStep::LCutin, vec![l_style.clone()])?;
                        ctx.lines_as(
                            "Veronica",
                            args![
                                ((Val::from("You have chosen style no. (") + l_style.clone()) + Val::from(").")),
                                "I shall proceed with your request.",
                                "Would you mind?"
                            ],
                        )?;
                        ctx.next()?;
                        if ctx.menu(&["No, I don't mind.", "Yes, let me choose another one."])? == 0 {
                            if ctx.call(Function::GetLook, args![6])? == 0 {
                                ctx.lines_as(
                                    "Veronica",
                                    args![
                                        "Oh, my, you haven't dyed your hair",
                                        "at all. You would look even more",
                                        "fabulous if you dyed your hair...",
                                        "Oh well, I will do it for free.",
                                        "So what kind of color would you like?"
                                    ],
                                )?;
                                ctx.next()?;
                                'b3: {
                                    let subject3 = Val::from(runtime::select_values(
                                        ctx,
                                        &[Val::from("Red.:Yellow.:Purple.:Orange.:Green.:Blue.:White.:Dark Brown.:Cancel.")],
                                    )?);
                                    let mut matched3 = false;
                                    let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                        && !subject3.loosely_equals(&Val::from(2))
                                        && !subject3.loosely_equals(&Val::from(3))
                                        && !subject3.loosely_equals(&Val::from(4))
                                        && !subject3.loosely_equals(&Val::from(5))
                                        && !subject3.loosely_equals(&Val::from(6))
                                        && !subject3.loosely_equals(&Val::from(7))
                                        && !subject3.loosely_equals(&Val::from(8))
                                        && !subject3.loosely_equals(&Val::from(9));
                                    if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(8);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(1);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(2);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(4)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(3);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(5)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(4);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(6)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(5);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(7)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(6);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(8)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        l_pallete = Val::from(7);
                                        break 'b3;
                                    }
                                    if !matched3 && subject3.loosely_equals(&Val::from(9)) {
                                        matched3 = true;
                                    }
                                    if matched3 {
                                        ctx.lines_as(
                                            "Veronica",
                                            args![
                                                "Oh, I was gonna do it for free.",
                                                "Well, if you change your mind, please come again.",
                                                "The color of your hair enhances your look."
                                            ],
                                        )?;
                                        ctx.close_window()?;
                                        return Err(Stop::End);
                                    }
                                }
                            }
                            ctx.call(Function::Nude, args![])?;
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "Now, let's get started. Try to",
                                    "stay still, dear. If you move,",
                                    "it might ruin the perfect look",
                                    "I intend to give you. Trust me,",
                                    "I will make you look fabulous~"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "- *snip snip snip snip* -",
                                    "- *bzzzzzzz bzzzzzzz bzzzzzzz bzzzzzzz* -",
                                    "- *snip snip snip snip* -",
                                    "- *bzzzzzzz bzzzzzzz bzzzzzzz bzzzzzzz* -"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.player().set_zeny(ctx.player().zeny()? - 99800)?;
                            ctx.items().take(973, 3)?;
                            ctx.items().take(974, 3)?;
                            ctx.items().take(901, 100)?;
                            ctx.items().take(1094, 100)?;
                            ctx.items().take(1020, 100)?;
                            ctx.items().take(1060, 100)?;
                            ctx.items().take(7152, 100)?;
                            ctx.call(Function::SetLook, vec![Val::from(1), l_style.clone()])?;
                            ctx.call(Function::SetLook, vec![Val::from(6), l_pallete.clone()])?;
                            ctx.fx().cutin("", 255)?;
                            ctx.lines_as(
                                "Veronica",
                                args![
                                    "Alright, it's done~",
                                    "I hope you like",
                                    ((Val::from("this style no.(") + l_style.clone()) + Val::from(").")),
                                    "Feel free to come back anytime",
                                    "when you want a new hairstyle. Hohohohohoho~"
                                ],
                            )?;
                            ctx.call(Function::SetLook, vec![Val::from(1), l_style.clone()])?;
                            ctx.call(Function::SetLook, vec![Val::from(6), l_pallete.clone()])?;
                            ctx.close_window()?;
                            return Err(Stop::End);
                        }
                        ctx.lines_as(
                            "Veronica",
                            args![
                                "Okay then, please choose one",
                                "a hairstyle again. I believe",
                                "you will find the look that's best for you."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                        matched1 = true;
                    }
                    if matched1 {
                        ctx.lines_as(
                            "Veronica",
                            args![
                                "Everybody deserves the right to",
                                "pursue beauty. I hope that you",
                                "will find the right hairstyle",
                                "one of these days."
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                step = HairDresserStep::LCutin;
                continue 'machine;
            }
            HairDresserStep::LCutin => {
                l_num = runtime::arg(&args, 0, Val::from(0));
                let sex = if ctx.var("Sex").get()? == constants::SEX_MALE { "m" } else { "f" };
                let zero_pad = if l_num.clone().number()? < 10 { "0" } else { "" };
                ctx.call(
                    Function::Cutin,
                    args![Val::from(format!("hair_{sex}_{zero_pad}")) + l_num.clone(), 4],
                )?;
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn hair_dresser(ctx: &Ctx) -> Script {
    hair_dresser_run(ctx, HairDresserStep::Start, Vec::new()).map(|_| ())
}

pub fn hair_dresser_li(ctx: &Ctx) -> Script {
    let mut l_headpalette = Val::from(0);
    let mut l_input = Val::from(0);
    ctx.lines_as(
        "Prince Shammi",
        args![
            "Welcome to Prince Shammi's",
            "Beauty Shop, the place to go",
            "for faaabulous hair. Don't be",
            "shy, tell me exactly how you",
            "want me to make you glamorous~"
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Check all hairstyles:Change hairstyle:Cancel")],
        )?);
        let mut matched1 = false;
        let no_case1 =
            !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2)) && !subject1.loosely_equals(&Val::from(3));
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Prince Shammi",
                args!["Oh, would you like to", "see all of the trendy new", "hairstyles I offer?"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Prince Shammi",
                args!["Please, oh please, choose from the following styles and I will show you a preview."],
            )?;
            ctx.next()?;
            'b2: {
                let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Old Hairstyles:New Hairstyles")])?);
                let mut matched2 = false;
                let no_case2 = !subject2.loosely_equals(&Val::from(1)) && !subject2.loosely_equals(&Val::from(2));
                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        'b3: {
                            let subject3 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Petite Style:Executioner Style:Prince Style:Deviace Style:Cancel")],
                            )?);
                            let mut matched3 = false;
                            let no_case3 = !subject3.loosely_equals(&Val::from(1))
                                && !subject3.loosely_equals(&Val::from(2))
                                && !subject3.loosely_equals(&Val::from(3))
                                && !subject3.loosely_equals(&Val::from(4))
                                && !subject3.loosely_equals(&Val::from(5));
                            if !matched3 && subject3.loosely_equals(&Val::from(1)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.fx().cutin("hair_m_20", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "This is the ^3131FFPetite Style^000000,",
                                        "which softens the gentleman's",
                                        "appearance with long braids",
                                        "for a fluffier appearance."
                                    ],
                                )?;
                                break 'b3;
                            }
                            if !matched3 && subject3.loosely_equals(&Val::from(2)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.fx().cutin("hair_m_21", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Oh, the ^3131FFExecutioner Style^000000!",
                                        "It's a rugged, shaggy style",
                                        "for that tough guy look that's",
                                        "becoming popular these days.",
                                        "And every girl loves a tough",
                                        "guy, right? ^333333*Tee hee~*^000000"
                                    ],
                                )?;
                                break 'b3;
                            }
                            if !matched3 && subject3.loosely_equals(&Val::from(3)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.fx().cutin("hair_m_22", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "You certainly have an",
                                        "eye for fashion! Yes, this",
                                        "is the ^3131FFPrince Style^000000, the",
                                        "pinnacle of sexiness and",
                                        "sophistication. Magnifique, no?",
                                        "Yes, choose this one, this one!"
                                    ],
                                )?;
                                break 'b3;
                            }
                            if !matched3 && subject3.loosely_equals(&Val::from(4)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.fx().cutin("hair_m_23", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "A-ha~! The ^3131FFDeviace Style^000000!",
                                        "This is much like the Prince",
                                        "Style, but with shorter hair",
                                        "in the back. Yes, this look",
                                        "is very neat and dandy."
                                    ],
                                )?;
                                break 'b3;
                            }
                            if !matched3 && subject3.loosely_equals(&Val::from(5)) {
                                matched3 = true;
                            }
                            if matched3 {
                                ctx.fx().cutin("hair_f_01", 255)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "No? You didn't want",
                                        "to take a look? Please,",
                                        "you're an adventurer, I know",
                                        "you can be more daring than",
                                        "that! Be fashionably adventurous, you fashionable adventurer~"
                                    ],
                                )?;
                                ctx.npc().emotion(constants::ET_THROB)?;
                                ctx.close_window()?;
                                ctx.fx().cutin("", 255)?;
                                return ctx.end();
                            }
                        }
                    } else {
                        'b4: {
                            let subject4 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Spring Rabbit Style:Harpy Style:Medusa Style:Isis Style:Cancel")],
                            )?);
                            let mut matched4 = false;
                            let no_case4 = !subject4.loosely_equals(&Val::from(1))
                                && !subject4.loosely_equals(&Val::from(2))
                                && !subject4.loosely_equals(&Val::from(3))
                                && !subject4.loosely_equals(&Val::from(4))
                                && !subject4.loosely_equals(&Val::from(5));
                            if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.fx().cutin("hair_f_20", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Oh yes, this is the ",
                                        "^3131FFSpring Rabbit Style^000000.",
                                        "The bobbing forelock",
                                        "adds an aura of chic,",
                                        "cutsiness and playfulness.",
                                        "Yes? No? Yes? No? Oh yes!"
                                    ],
                                )?;
                                break 'b4;
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.fx().cutin("hair_f_21", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Ooh, are you interested",
                                        "in the ^3131FFHarpy Style^000000? The",
                                        "natural curl coupled with",
                                        "the pony tail results in",
                                        "a sophisticated, yet very",
                                        "natural and relaxed look~"
                                    ],
                                )?;
                                break 'b4;
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(3)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.fx().cutin("hair_f_22", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Ahh, the ^3131FFMedusa Style^000000~",
                                        "These boldy flowing locks",
                                        "scream power and dominance",
                                        "and is ideal for the big career",
                                        "woman who wishes to be...",
                                        "irresistable to men~"
                                    ],
                                )?;
                                break 'b4;
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(4)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.fx().cutin("hair_f_23", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Ooh, the ^3131FFIsis Style^000000~",
                                        "Yes, you'll look very cute",
                                        "with your hair in buns on",
                                        "on both sides of your head.",
                                        "It'll be very darling on you!"
                                    ],
                                )?;
                                break 'b4;
                            }
                            if !matched4 && subject4.loosely_equals(&Val::from(5)) {
                                matched4 = true;
                            }
                            if matched4 {
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "No? You didn't want",
                                        "to take a look? Please,",
                                        "you're an adventurer, I know",
                                        "you can be more daring than",
                                        "that! Be fashionably adventurous, you fashionable adventurer~"
                                    ],
                                )?;
                                ctx.npc().emotion(constants::ET_THROB)?;
                                return ctx.close();
                            }
                        }
                    }
                    break 'b2;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                    matched2 = true;
                }
                if matched2 {
                    if ctx.var("Sex").get()? == constants::SEX_MALE {
                        'b5: {
                            let subject5 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Emergency Heal Perm:Aura Blade Cut:Power Swing:Renovatio Cut:Cancel")],
                            )?);
                            let mut matched5 = false;
                            let no_case5 = !subject5.loosely_equals(&Val::from(1))
                                && !subject5.loosely_equals(&Val::from(2))
                                && !subject5.loosely_equals(&Val::from(3))
                                && !subject5.loosely_equals(&Val::from(4))
                                && !subject5.loosely_equals(&Val::from(5));
                            if !matched5 && subject5.loosely_equals(&Val::from(1)) {
                                matched5 = true;
                            }
                            if matched5 {
                                ctx.fx().cutin("hair_m_24", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "This is the ^3131FFEmergency Heal Perm^000000",
                                        "It is quite popular among the healing class."
                                    ],
                                )?;
                                break 'b5;
                            }
                            if !matched5 && subject5.loosely_equals(&Val::from(2)) {
                                matched5 = true;
                            }
                            if matched5 {
                                ctx.fx().cutin("hair_m_25", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "You must be after a lady yes?",
                                        "The ^3131FFAura Blade Cut^000000",
                                        "is known to make the ladies swoon, you tiger you!"
                                    ],
                                )?;
                                break 'b5;
                            }
                            if !matched5 && subject5.loosely_equals(&Val::from(3)) {
                                matched5 = true;
                            }
                            if matched5 {
                                ctx.fx().cutin("hair_m_26", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Oh you brute!",
                                        "^3131FFPower Swing Cut^000000",
                                        "Flex your style muscles with this hairstyle. This is definitely your look."
                                    ],
                                )?;
                                break 'b5;
                            }
                            if !matched5 && subject5.loosely_equals(&Val::from(4)) {
                                matched5 = true;
                            }
                            if matched5 {
                                ctx.fx().cutin("hair_m_27", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Ah! I see you're only interested in the latest trends.",
                                        "Straight from the runway is the ^3131FFRenovatio Cut^000000."
                                    ],
                                )?;
                                break 'b5;
                            }
                            if !matched5 && subject5.loosely_equals(&Val::from(5)) {
                                matched5 = true;
                            }
                            if matched5 {
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "No? You didn't want",
                                        "to take a look? Please,",
                                        "you're an adventurer, I know",
                                        "you can be more daring than",
                                        "that! Be fashionably adventurous, you fashionable adventurer~"
                                    ],
                                )?;
                                ctx.npc().emotion(constants::ET_THROB)?;
                                return ctx.close();
                            }
                        }
                    } else {
                        'b6: {
                            let subject6 = Val::from(runtime::select_values(
                                ctx,
                                &[Val::from("Assumptio Perm:Soul Changer Cut:X Tornado Cut:Oratio Cut:Cancel")],
                            )?);
                            let mut matched6 = false;
                            let no_case6 = !subject6.loosely_equals(&Val::from(1))
                                && !subject6.loosely_equals(&Val::from(2))
                                && !subject6.loosely_equals(&Val::from(3))
                                && !subject6.loosely_equals(&Val::from(4))
                                && !subject6.loosely_equals(&Val::from(5));
                            if !matched6 && subject6.loosely_equals(&Val::from(1)) {
                                matched6 = true;
                            }
                            if matched6 {
                                ctx.fx().cutin("hair_f_24", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "This is the ^3131FFAssumptio Perm^000000",
                                        "It's a shorter style perm that allows for maximum spellcasting."
                                    ],
                                )?;
                                break 'b6;
                            }
                            if !matched6 && subject6.loosely_equals(&Val::from(2)) {
                                matched6 = true;
                            }
                            if matched6 {
                                ctx.fx().cutin("hair_f_25", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "You must be a man killer no?",
                                        "The ^3131FFSoul Changer Cut^000000",
                                        "will make any man open his wall... er heart to you!"
                                    ],
                                )?;
                                break 'b6;
                            }
                            if !matched6 && subject6.loosely_equals(&Val::from(3)) {
                                matched6 = true;
                            }
                            if matched6 {
                                ctx.fx().cutin("hair_f_26", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "This is a bit of a trendy style",
                                        "^3131FFX Tornado Cut^000000",
                                        "It's for adventurous people who like change."
                                    ],
                                )?;
                                break 'b6;
                            }
                            if !matched6 && subject6.loosely_equals(&Val::from(4)) {
                                matched6 = true;
                            }
                            if matched6 {
                                ctx.fx().cutin("hair_f_27", 4)?;
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "Ah! I see you're only interested in the latest trends.",
                                        "Straight from the runway is the ^3131FFOratio Cut^000000.",
                                        "You'll be the envy of all of your friends with this hairstyle."
                                    ],
                                )?;
                                break 'b6;
                            }
                            if !matched6 && subject6.loosely_equals(&Val::from(5)) {
                                matched6 = true;
                            }
                            if matched6 {
                                ctx.lines_as(
                                    "Prince Shammi",
                                    args![
                                        "No? You didn't want",
                                        "to take a look? Please,",
                                        "you're an adventurer, I know",
                                        "you can be more daring than",
                                        "that! Be fashionably adventurous, you fashionable adventurer~"
                                    ],
                                )?;
                                ctx.npc().emotion(constants::ET_THROB)?;
                                return ctx.close();
                            }
                        }
                    }
                }
            }
            ctx.close_window()?;
            ctx.fx().cutin("", 255)?;
            return ctx.end();
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            if ctx.player().base_level()? < 60 {
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "Oh, I'm so sorry, but",
                        "I can only perform my",
                        "services for clients that have",
                        "matured enough to find their",
                        "true inner beauty. But please",
                        "come back once you do, okay?"
                    ],
                )?;
                return ctx.close();
            } else if (((((((ctx.items().count(973)? < 3 || ctx.items().count(974)? < 3) || ctx.items().count(901)? < 100)
                || ctx.items().count(1094)? < 100)
                || ctx.items().count(1020)? < 100)
                || ctx.items().count(1060)? < 100)
                || ctx.items().count(7152)? < 100)
                || ctx.player().zeny()? < 99800)
            {
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "If you've already decided",
                        "what hairstyle you'd like,",
                        "please have my service charge",
                        "ready, as well as the materials",
                        "I will need in performing this",
                        "service, okay? Please bring..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "^3355FF3 Counteragent^000000,",
                        "^3355FF3 Mixture^000000,",
                        "^3355FF100 Daenggie^000000,",
                        "^3355FF100 Short Daenggie^000000..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "^3355FF100 Black Hair^000000,",
                        "^3355FF100 Golden Hair^000000,",
                        "^3355FF100 Glossy Hair^000000",
                        "and ^3355FF99,800 zeny^000000.",
                        "Once you do that, I'll make",
                        "a miracle out of your hair!"
                    ],
                )?;
                return ctx.close();
            }
            ctx.lines_as(
                "Prince Shammi",
                args![
                    "Alright, please choose",
                    "which hairstyle you wish",
                    "to have from numbers 20 to 25.",
                    "Here's a list of the style names just in case you need them~"
                ],
            )?;
            ctx.next()?;
            ctx.mes("[Prince Shammi]")?;
            if ctx.var("Sex").get()? == constants::SEX_MALE {
                ctx.lines(args![
                    "No. 20: Petite Style",
                    "No. 21: Executioner Style",
                    "No. 22: Prince Style",
                    "No. 23: Deviace Style",
                    "No. 24: Emergency Heal Perm",
                    "No. 25: Aura Blade Cut",
                    "No. 26: Power Swing and",
                    "No. 27: Renovatio Cut."
                ])?;
            } else {
                ctx.lines(args![
                    "No. 20: Spring Rabbit Style",
                    "No. 21: Harpy Style",
                    "No. 22: Medusa Style",
                    "No. 23: Isis Style",
                    "No. 24: Assumptio Perm",
                    "No. 25: Soul Changer Cut",
                    "No. 26: X Tornado Cut and",
                    "No. 27: Oratio Cut."
                ])?;
            }
            ctx.next()?;
            let (input, status) = runtime::input_number(ctx, None, None)?;
            l_input = input;
            if l_input.clone() == 0 {
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "Oh...?",
                        "You decided to cancel?",
                        "Well, you know what's",
                        "best for you, I suppose.",
                        "Still, I'm so disappointed~"
                    ],
                )?;
                return ctx.close();
            } else if (l_input.clone().number()? < 20 || l_input.clone().number()? > 27) {
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "Dearie, please enter",
                        "a number from ''20'' to",
                        "''25,'' alright? Then I can",
                        "get right to work at making",
                        "you soooooooo beautiful!"
                    ],
                )?;
                return ctx.close();
            } else if ctx
                .call(Function::GetLook, args![constants::VAR_HEAD])?
                .loosely_equals(&l_input.clone())
            {
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "Oh dear me, you're not",
                        "going to waste money for",
                        "the same hairstyle that you",
                        "have now, are you? You can",
                        "have someone else change",
                        "your hair color, you know."
                    ],
                )?;
                return ctx.close();
            } else {
                let sex = if ctx.var("Sex").get()? == constants::SEX_MALE { "m" } else { "f" };
                ctx.call(
                    Function::Cutin,
                    args![Val::from(format!("hair_{sex}_")) + l_input.clone() + Val::from(".BMP"), 4],
                )?;
                ctx.lines_as(
                    "Prince Shammi",
                    args![
                        "Oooh! Now, is this the",
                        "hairstyle that you wanted?",
                        ((Val::from("This is No. ") + l_input.clone()) + Val::from(", by the way."))
                    ],
                )?;
                ctx.next()?;
                'b7: {
                    let subject7 = Val::from(runtime::select_values(ctx, &[Val::from("Yes.:No.")])?);
                    let mut matched7 = false;
                    let no_case7 = !subject7.loosely_equals(&Val::from(1)) && !subject7.loosely_equals(&Val::from(2));
                    if !matched7 && subject7.loosely_equals(&Val::from(1)) {
                        matched7 = true;
                    }
                    if matched7 {
                        if ctx.call(Function::GetLook, args![constants::VAR_HEADPALETTE])? == 0 {
                            ctx.lines_as(
                                "Prince Shammi",
                                args![
                                    "Oh, Sweet Christmas,",
                                    "I almost forgot! Would",
                                    "you like me to dye your",
                                    "hair, free of charge? It's",
                                    "a part of my service, so",
                                    "please choose a color~"
                                ],
                            )?;
                            ctx.next()?;
                            'b8: {
                                let subject8 = Val::from(runtime::select_values(
                                    ctx,
                                    &[Val::from("Red:Yellow:Purple:Orange:Green:Blue:White:Dark Brown")],
                                )?);
                                let mut matched8 = false;
                                let no_case8 = !subject8.loosely_equals(&Val::from(1))
                                    && !subject8.loosely_equals(&Val::from(2))
                                    && !subject8.loosely_equals(&Val::from(3))
                                    && !subject8.loosely_equals(&Val::from(4))
                                    && !subject8.loosely_equals(&Val::from(5))
                                    && !subject8.loosely_equals(&Val::from(6))
                                    && !subject8.loosely_equals(&Val::from(7))
                                    && !subject8.loosely_equals(&Val::from(8));
                                if !matched8 && subject8.loosely_equals(&Val::from(1)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(8);
                                    break 'b8;
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(2)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(1);
                                    break 'b8;
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(3)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(2);
                                    break 'b8;
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(4)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(3);
                                    break 'b8;
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(5)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(4);
                                    break 'b8;
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(6)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(5);
                                    break 'b8;
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(7)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(6);
                                    break 'b8;
                                }
                                if !matched8 && subject8.loosely_equals(&Val::from(8)) {
                                    matched8 = true;
                                }
                                if matched8 {
                                    l_headpalette = Val::from(7);
                                    break 'b8;
                                }
                            }
                        }
                        ctx.lines_as(
                            "Prince Shammi",
                            args![
                                "Okay, let's get",
                                "started, shall we?",
                                "Keep your head still,",
                                "now. Yes, that's good..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines(args![
                            "^3355FF*Snip snip*",
                            "*Rustle rustle*",
                            "*Clip clip clip clip*",
                            "*Bzzzzzzzzzzzzzzzzzzzz*^000000"
                        ])?;
                        ctx.next()?;
                        ctx.player().set_zeny(ctx.player().zeny()? - 99800)?;
                        ctx.items().take(973, 3)?;
                        ctx.items().take(974, 3)?;
                        ctx.items().take(901, 100)?;
                        ctx.items().take(1094, 100)?;
                        ctx.items().take(1020, 100)?;
                        ctx.items().take(1060, 100)?;
                        ctx.items().take(7152, 100)?;
                        ctx.call(Function::SetLook, vec![Val::from(constants::VAR_HEAD), l_input.clone()])?;
                        ctx.call(
                            Function::SetLook,
                            vec![Val::from(constants::VAR_HEADPALETTE), l_headpalette.clone()],
                        )?;
                        ctx.lines_as(
                            "Prince Shammi",
                            args![
                                "Well, we're all finished!",
                                "And my, oh my, you look even",
                                "more fabulous that I thought",
                                "you would! Oh, I can't believe"
                            ],
                        )?;
                        if ctx.var("Sex").get()? == constants::SEX_MALE {
                            ctx.lines(args!["how tough and elegant you are~", "So ruggedly manly and handsome!"])?;
                        } else {
                            ctx.lines(args!["how graceful and elegant you", "look! Absolutely gorgeous!"])?;
                        }
                        ctx.npc().emotion(constants::ET_CHUP)?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Prince Shammi",
                            args![
                                "You love your new",
                                "hair, don't you? Feel",
                                "free to come back anytime.",
                                "I'll make you the best looking",
                                "person in the entire world!"
                            ],
                        )?;
                        ctx.npc().emotion(constants::ET_BEST)?;
                        ctx.close_window()?;
                        ctx.fx().cutin("", 255)?;
                        return ctx.end();
                    }
                    if !matched7 && subject7.loosely_equals(&Val::from(2)) {
                        matched7 = true;
                    }
                    if matched7 {
                        ctx.lines_as(
                            "Prince Shammi",
                            args![
                                "Oh, did you forget which",
                                "hairstyle goes with which",
                                "number? By all means, please",
                                "check again! Find the one that",
                                "is perfect just for you, okay?"
                            ],
                        )?;
                        ctx.close_window()?;
                        ctx.fx().cutin("", 255)?;
                        return ctx.end();
                    }
                }
            }
            break 'b1;
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Prince Shammi",
                args!["Humm ? ", "Maybe you don't understand", "my futuristic styles.", "Goodbye! "],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}
