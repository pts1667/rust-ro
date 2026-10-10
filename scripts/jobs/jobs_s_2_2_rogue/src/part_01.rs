use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn thug_rg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Thug",
        args![
            "*Sigh...*",
            "What is life?",
            "And what use",
            "is money? ...Damn.",
            "Damn this worthless life!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Thug", args!["Hey, kid.", "What the hell", "are you lookin' at?"])?;
    ctx.next()?;
    if Val::from(runtime::select_values(ctx, &[Val::from("Me? N-nothing!':........")])?) == 1 {
        ctx.lines_as(
            "Thug",
            args!["Then get the", "hell out of my face!", "Didn't you hear me?", "Get lost!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Thug",
        args![
            "Hmmm...",
            "Maybe I'll swing by the ^0000FFRogue Guild^000000 in ^0000FFParos Lighthouse^000000..."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Thug", args!["I needz my money,", "and they best have it..."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn thug_rg(ctx: &Ctx) -> Script {
    thug_rg_body(ctx, Vec::new()).map(|_| ())
}

fn rogue_guildsman_rg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_assassin_t = Val::from(0);
    let mut l_jlevel = Val::from(0);
    if ctx.var("Upper").get()? == 1 {
        ctx.lines_as("Markie", args!["Eh? You...you...?!", "Hey, haven't we met before?"])?;
        ctx.next()?;
        ctx.lines_as(
            "Markie",
            args![
                "..............",
                "Awww, ^FF0000I am sorry^000000! I think I misunderstood you from someone I know."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Markie", args![".......", "........It is strange though. Umm."])?;
        ctx.next()?;
        ctx.lines_as("Markie", args!["I never misunderstand people...oh well, be safe anyway!"])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_THIEF")?) {
        if ctx.var("SkillPoint").get()?.is_true() {
            ctx.lines_as("Rogue Guildsman", args!["Yo, what are you doin'?!", "You can't change your job if you got unused skill points, so use 'em all up. You bettah check yo-self before you wreck yo-self."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        if ctx.var("JobLevel").get()?.number()? > 39 {
            if ctx.var("rogue_q").get()? == 0 {
                ctx.lines_as("Rogue Guildsman", args!["So what's a kid", "like you doin' here?"])?;
                if ctx.var("Sex").get()?.loosely_equals(&ctx.constant("SEX_MALE")?) {
                    ctx.lines(args!["Oh, I get it now...", "The widdle boy wants"])?;
                } else {
                    ctx.lines(args!["Oh, I see...", "Lil' cutie wants "])?;
                }
                ctx.mes("to be a ^800000Rogue^000000.")?;
                ctx.next()?;
                ctx.lines_as("Rogue Guildsman", args!["Eh, nice meetin' you, I guess. I'm Markie, and I do work for the Rogue Guild, a philanthro-- *ahem* a ^800000feelanthropist^000000 group, as you can see. So what's your name?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Markie",
                    args![
                        ((Val::from("...") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("?")),
                        "Heh heh! Cool name.",
                        "If it was dorky, we'd",
                        "make you change it,",
                        "so you're in luck."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["So why you wanna join up with the Rogues? I guess you gave me your real name, so you'd be an honest Rogue. Not many of those around, heh heh~"])?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["Rule number 1 for Rogues...", "Never give out your real identity to most people, most of the time. It's just a little backup measure we like to use."])?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["Right. I'm officially accepting your application, so now you gotta take a test. Don't sweat it, this first one is real simple."])?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["Alright...", "Let's get started!"])?;
                ctx.next()?;
            } else if ctx.var("rogue_q").get()? == 1 {
                ctx.lines_as(
                    "Markie",
                    args![
                        "You again?",
                        "Okay, you probably screwed up last time 'cuz you were way too nervous. So just chill and pass",
                        "this test, okay?"
                    ],
                )?;
                ctx.next()?;
            } else if ctx.var("rogue_q").get()? == 2 {
                ctx.lines_as("Markie", args!["Go talk to Smith. His test might be pretty hard. He's one of the guys who makes sure that people pay up their debts to us. So yeah, he might be a bit of a hard case."])?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["Yeah...", "That guy can be pretty anal, but we need a guy like him in our guild. Anyway, be careful. Lots of luck to you, pal."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("rogue_q").get()?.number()? > 2 && ctx.var("rogue_q").get()?.number()? < 16) {
                ctx.lines_as("Markie", args!["Hey yo...", "Do your best."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Markie",
                    args![
                        "Heh heh...",
                        "Fresh meat. This'll be",
                        "a cinch to--Wait! Er, I wasn't talkin' about you! I meant the other fresh meat~"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else if (ctx.var("rogue_q").get()? == 16 || ctx.var("rogue_q").get()? == 17) {
                ctx.call(Function::ChangeQuest, vec![Val::from(2026), Val::from(2027)])?;
                ctx.mes("[Markie]")?;
                if ctx.var("rogue_q").get()? == 16 {
                    ctx.lines(args![
                        "Oh hey, it's you!",
                        "You did a good job, guy.",
                        "Now, lemme change your",
                        "job to Rogue. You earned it!"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Markie", args!["Congrats~!", "You look", "sooo dope!"])?;
                } else {
                    ctx.lines(args![
                        "Oh! It's you!",
                        "You were actually able to put up with that guy? Good stuff! Must've had a rough time collect all those items, eh?"
                    ])?;
                    ctx.next()?;
                    ctx.lines_as("Markie", args!["Hey hey~", "Congrats!", "You've been", "doin' a great job~"])?;
                }
                l_jlevel = ctx.var("JobLevel").get()?;
                shared::other_global_functions::job_change(ctx, vec![ctx.constant("JOB_ROGUE")?])?;
                shared::other_global_functions::f_clearjobvar(ctx, vec![])?;
                ctx.call(Function::CompleteQuest, vec![Val::from(2027)])?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["Now...", "It's time", "for me to make", "a speech~ *Ahem*"])?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["Enjoy your freedom as a Rogue. Just remember that you gotta be free and responsible at the same time. So treat other guys the way you wanna be treated, kay? Alright, seeya round."])?;
                ctx.close_window()?;
                if l_jlevel.clone() == 50 {
                    ctx.call(Function::GetItem, vec![Val::from(1220), Val::from(1)])?;
                } else {
                    ctx.call(Function::GetItem, vec![Val::from(1219), Val::from(1)])?;
                }
                return Err(Stop::End);
            }
            if Val::from(runtime::select_values(
                ctx,
                &[Val::from("I'm ready.:Hold on, I need to get ready!")],
            )?) == 2
            {
                ctx.lines_as(
                    "Markie",
                    args![
                        "Get ready...?",
                        "Fine, fine.",
                        "Take your sweet",
                        "time, why don't you?",
                        "But hurry up and",
                        "come back, got it?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            ctx.next()?;
            ctx.lines_as(
                "Markie",
                args![
                    "Listen carefully, and",
                    "pick the right answer.",
                    "Capish? Now, lemme",
                    "read these questions..."
                ],
            )?;
            ctx.next()?;
            let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
            if subject1 == 1 {
                ctx.lines_as(
                    "Markie",
                    args!["1. Choose the skill necessary for learning ^880000Stalk^000000."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "^880000Hiding^000000:^880000Steal^000000:^880000Improve Dodge^000000:^880000Bash^000000",
                    )],
                )?) == 1
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["2. In comparison to the Merchant's Level 10 ^880000Discount^000000 skill, how much more of a discount, in terms of percent, can a Rogue get with Level 10 ^880000Haggle^000000 skill?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("3 %:2 %:1 %:0 %")])?) == 3 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["3. What is the correct description for the skill, ^880000Mug^000000?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Steal items from players:Steal items from monsters:Steal Zeny from monsters:Steal Zeny from players",
                    )],
                )?) == 3
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["4. How many Rogues does it require to activate the skill, ^880000Slyness^000000?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("1 Rogues + 2 Assassin:1 Thief + 2 Rogue:4 Thieves:2 Rogues")],
                )?) == 4
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["5. Choose the skill that you can learn at Level 5 ^880000Divest Helm^000000."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "^880000Envenom^000000:^880000Strip Tease^000000:^880000Venom Splasher^000000:^880000Divest Shield^000000",
                    )],
                )?) == 4
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["6. Choose the skill which allows its user to move while hiding."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "^880000Hiding^000000:^880000Back Slide^000000:^880000Stalk^000000:^880000Sand Attack^000000",
                    )],
                )?) == 3
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["7. Choose the card that increases the accuracy rate of its owner."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Andre Card.:Familiar Card.:Mummy Card.:Marina Card.")],
                )?) == 3
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["8. Choose the monster that receives more damage when it's attacked by a weapon with the Vadon card (20 % more damage on Fire property)."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Vadon:Deviruchi:Elder Willow:Baphomet")],
                )?) == 3
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["9. How much SP does the skill ^880000Double Attack^000000 require when used with a Dagger?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("15:Passive skill, no SP required.:Passive skill, 10 SP:54")],
                )?) == 2
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["10. Choose the most efficient dagger to use in the Byalan Dungeon."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Wind Main-Gauche:Ice Main-Gauche:Earth Main-Gauche:Fire Main-Gauche")],
                )?) == 1
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
            } else if subject1 == 2 {
                ctx.lines_as("Markie", args!["1. Which monster drops a slotted Gladius?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Thief Bug:Peco Peco:Desert Wolf:Kobold")],
                )?) == 4
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["2. Which monster drops a slotted Main-Gauche?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Hornet:Desert Wolf:Marionette:Myst")])?) == 1 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["3. Choose the class that is able to create unique potions."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Merchant:Alchemist:Blacksmith:Priest")],
                )?) == 2
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["4. Choose the weapon that Rogues aren't allowed to use."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Gakkung:Crossbow:Gladius:Katar")])?) == 4 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["5. Choose the property that the monster Hode possesses."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Water:Fire:Wind:Earth")])?) == 4 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["6. Choose the monster that is unable to be tamed for as a Cute Pet."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Poporing:Creamy:Orc:Poison Spore")])?) == 2 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["7. Choose the monster that receives more damage from a Dagger with the Fire property."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Dagger Goblin:Mace Goblin:Morning Star Goblin:Hammer Goblin")],
                )?) == 4
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["8. Choose the town that doesn't have any guild castles."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Prontera:Al De Baran:Alberta:Payon")])?) == 3 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["9. Choose the plant that drops Blue Herbs."])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Green Plant:Yellow Plant:Blue Plant:Shining Plant")])? {
                    3 => {
                        l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                    }
                    4 => {
                        l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Markie",
                    args!["10. Choose the monster that does not have the Undead property."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Zombie:Megalodon:Familiar:Khalitzburg")],
                )?) == 3
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
            } else if subject1 == 3 {
                ctx.lines_as(
                    "Markie",
                    args!["1. By what percentage is the flee rate increased when a Thief masters the ^880000Improve Dodge^000000?"],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("30:40:160:20")])?) == 1 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["2. Choose the monster that detects a characters using the Hiding or Cloaking skill."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Worm Tail:Argos:Mummy:Soldier Skeleton")],
                )?) == 2
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["3. Choose the location where Thieves can change their jobs to Rogues."],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Comodo:Kokomo Beach:Paros Lighthouse:Morocc")],
                )?) == 3
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["4. In which town can Novices change their jobs to Thieves?"])?;
                ctx.next()?;
                if Val::from(runtime::select_values(ctx, &[Val::from("Comodo:Lutie:Alberta:Morocc")])?) == 4 {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["5. Choose the card that does not affect the DEX stat."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Rocker Card:Mummy Card:Zerom Card:Drops Card")],
                )?) == 2
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as("Markie", args!["6. So what's cool about being a Rogue?"])?;
                ctx.next()?;
                let choice = runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Being totally badass.:The clothes, the style.:Getting to call other people, 'foo'':Excellent attack strength",
                    )],
                )?;
                ctx.var("@menu").set(choice)?;
                l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                ctx.lines_as("Markie", args!["7. When is it possible to change jobs from Thief to Rogue?"])?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from("At job Level 30:At job Level 35:At Job Level 40:At Job Level 50")],
                )? {
                    3 => {
                        l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                    }
                    4 => {
                        l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Markie",
                    args![
                        "8. You want to dye your hair blue. What town do you go to, and in which direction, with 12 o' clock being North."
                    ],
                )?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Morocc, 7 o'clock:Prontera, 7 o'clock:Morocc, 5 o'clock:Prontera, 1 o'clock",
                    )],
                )?) == 2
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
                ctx.lines_as(
                    "Markie",
                    args!["9. Choose the mushroom that is required on the Thief job change quest."],
                )?;
                ctx.next()?;
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Orange Gooey Mushroom:Red Hairy Mushroom:Orange Net Mushroom:Orange Sticky Mushroom",
                    )],
                )? {
                    1 | 3 => {
                        l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                    }
                    _ => {}
                }
                ctx.lines_as("Markie", args!["10. Choose the card that least benefits the Rogue class."])?;
                ctx.next()?;
                if Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("Whisper Card:Elder Willow Card:Zerom Card:Matyr Card")],
                )?) == 2
                {
                    l_assassin_t = (l_assassin_t.clone() + Val::from(10));
                }
            }
            ctx.lines_as("Markie", args!["*Whew~*", "Finally.", "We're done."])?;
            ctx.next()?;
            ctx.lines_as(
                "Markie",
                args![
                    "Let's see.",
                    "You got...",
                    ((Val::from("") + l_assassin_t.clone()) + Val::from(" points."))
                ],
            )?;
            if l_assassin_t.clone().number()? > 80 {
                ctx.var("rogue_q").set(Val::from(2))?;
                ctx.call(Function::SetQuest, vec![Val::from(2017)])?;
                ctx.lines(args!["Good. You passed.", "We don't gotta", "do that again."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Markie",
                    args!["But don't get too comfortable just yet, you got more o' these tests. Your next one will be from Smith."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Markie",
                    args![
                        "So...",
                        "Go find Smith and finish up this test, yeah? Be careful though, Smith's a pretty anal guy."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            } else {
                ctx.var("rogue_q").set(Val::from(1))?;
                ctx.mes("Aw crud... You failed!")?;
                ctx.next()?;
                ctx.lines_as(
                    "Markie",
                    args!["Man, you shoulda learned more when you had the chance. Thanks for wasting my time."],
                )?;
                ctx.next()?;
                ctx.lines_as("Markie", args!["*Sigh...* Lemme give you some tips, yeah? I'm supposed to tell you about some kind of ^990000iro.ragnarokonline.com^000000 to help you learn what you need to know."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Markie",
                    args!["Of course, I don't know what the heck it means, but if you understand it, it'll probably help you out a lot."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        } else if ctx.var("JobLevel").get()?.number()? < 40 {
            ctx.lines_as("Rogue Guildsman", args!["Whoa, slow down newbie. We only accept people who are at least Thief Job Level 40. I ain't risking myself by letting you in before you're ready. Got it?"])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ASSASSIN")?) {
        ctx.lines_as(
            "Rogue Guildsman",
            args![
                "Huh...?",
                "What's an Assassin doin' here? Uh, you haven't been assigned to kill someone in the Rogue Guild, are you?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Rogue Guildsman",
            args!["In any case, don't mess with us! You can't catch me... I'm a smooth criminal!"],
        )?;
        ctx.next()?;
        ctx.lines_as("Rogue Guildsman", args!["Don't get it, huh? It's something I always used to say to Huey. If you're in the Assassin Guild, you oughta have met him..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("BaseJob").get()?.loosely_equals(&ctx.constant("JOB_ROGUE")?) {
        ctx.lines_as(
            "Markie",
            args![
                "Hey hey~",
                "Long time no see.",
                "Eh, right now we don't",
                "any requests from the",
                "guild for you, so just",
                "check back again later."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Rogue Guildsman",
            args![
                "Hey you...",
                "Get your ugly",
                "ass out of here",
                "before I redecorate",
                "that face of yours!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn rogue_guildsman_rg(ctx: &Ctx) -> Script {
    rogue_guildsman_rg_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum MrSmithRgStep {
    Start,
    SReq,
    SCheckItems,
}

fn mr_smith_rg_run(ctx: &Ctx, mut step: MrSmithRgStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_amount: Vec<Val> = Vec::new();
    let mut l_item_need = Val::from(0);
    let mut l_item_req: Vec<Val> = Vec::new();
    let mut l_var = Val::from(0);
    'machine: loop {
        match step {
            MrSmithRgStep::Start => {
                if ctx.var("rogue_q").get()? == 2 {
                    ctx.lines_as(
                        "Mr. Smith",
                        args![
                            "Welcome to",
                            "the Rogue guild.",
                            "From here on, I will",
                            "verify your qualification."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Smith",
                        args!["Before we get started,", "I want you to know", "about something..."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Smith",
                        args!["All new Rogues are required to pay an application fee, so I hope you take care of that first."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Mr. Smith", args!["What you have to understand is that the Rogue Guild does a lot of business, ^666666sometimes illegally^000000, that needs financial backup."])?;
                    ctx.next()?;
                    l_item_need = ctx.call(Function::Rand, vec![Val::from(1), Val::from(15)])?;
                    if (l_item_need.clone().number()? > 0 && l_item_need.clone().number()? < 6) {
                        mr_smith_rg_run(
                            ctx,
                            MrSmithRgStep::SReq,
                            vec![
                                Val::from("10 Skel-bone"),
                                Val::from("6 Blue Herb"),
                                Val::from("10 Decayed Nail"),
                                Val::from("10 Horrendous Mouth"),
                                Val::from(3),
                            ],
                        )?;
                    } else if (l_item_need.clone().number()? > 5 && l_item_need.clone().number()? < 11) {
                        mr_smith_rg_run(
                            ctx,
                            MrSmithRgStep::SReq,
                            vec![
                                Val::from("10 Green Herb"),
                                Val::from("10 Crab Shell"),
                                Val::from("10 Snake Scale"),
                                Val::from("10 Garlet"),
                                Val::from(4),
                            ],
                        )?;
                    } else if (l_item_need.clone().number()? > 10 && l_item_need.clone().number()? < 15) {
                        mr_smith_rg_run(
                            ctx,
                            MrSmithRgStep::SReq,
                            vec![
                                Val::from("10 Yellow Herb"),
                                Val::from("10 Shell"),
                                Val::from("10 Grasshopper's Leg"),
                                Val::from("10 Bear's Footskin"),
                                Val::from(5),
                            ],
                        )?;
                    } else if l_item_need.clone() == 15 {
                        ctx.lines_as("Mr. Smith", args!["I will let you know..."])?;
                        ctx.var("rogue_q").set(Val::from(6))?;
                        ctx.call(Function::ChangeQuest, vec![Val::from(2017), Val::from(2021)])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["I will let you know......"])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["I will let you know........", "By the way....."])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["Oh man...", "This is...", "Damn...", "Annoying!"])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["..."])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["...", "......"])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["...", "......", "........."])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["Today, I'm in a pissed off mood, ya' know why?! I haven't collected any bills! God! Idiot Thieves coming at me all the time, wanting to become Rogues!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["Jesus! Now I understand why our leader told us that working as customer support sucks."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["That god damn guild master assigned me to this shitty job. I'm better than this F$@king job!"],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("That god damn guild master assigned me to this shitty job! That Bastard!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.mes("That bastard should go to F$@king hell, I'm gonna kick that mother F$@kers ass! F#%k F#%k F#%k !!!")?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from(
                                    "That bastard should go to F$@king hell, I'm gonna kick that mother F$@kers ass! F#%k F#%k F#%k !!!",
                                ),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["That dipshit who just tried to change his job, the one before you..."],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("That dipshit who just tried to change his job, the one before you..."),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.mes("You know what the f#@k he talked to me about?!?")?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("You know what the f#@k he talked to me about?!?"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.mes("F#$@%#$*#$%@#$!!")?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![Val::from("in_rogue"), Val::from("F#$@%#$*#$%@#$!!"), ctx.constant("BC_MAP")?],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["What the--?! What's with this chat filter?! Stop #*!@$ing me! you stupid F#$%*! Let me talk!!!"],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("What the--?! What's with this chat filter?! Stop #*!@$ing me! you stupid F#$%*! Let me talk!!!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args![
                                ((Val::from("What the f#@k you looking at...? ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from("? That's your name!?"))
                            ],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                ((Val::from("What the f#@k you looking at...? ") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                    + Val::from("? That's your name!?")),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.lines(args![
                            " ",
                            ((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?) + Val::from("]")),
                            "Umm...",
                            "Sir...?",
                            "I didn't mean to make you upset. I just came here to so I could become a Rogue."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["Holy shit on a stick, what the f#$k was I just f!#king talking about, you moron!"],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("Holy shit on a stick, what the f#$k was I just f!#king talking about, you moron!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["Just leave me alone! Just leave alone! Just leave me alone!"],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("Just leave me alone! Just leave alone! Just leave me alone!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["Do whatever you want, okay? Just do whatever the F$!!K you want...!!"],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("Do whatever you want, okay? Just do whatever the F$!!K you want...!!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["Your application fee... ^FF000010,000 zeny^000000!!"])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args![
                                "^FF00005 Crysalis^000000!",
                                "^FF00005 Empty Bottle^000000!",
                                "^FF00005 Iron Ore^000000!",
                                "^FF00005 Stone Heart^000000!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args![
                                "^FF00005 Red Herb^000000!",
                                "^FF00005 Animal Skin^000000!!",
                                "^FF00005 Yellow Gemstone^000000!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args![
                                "^FF00005 Tooth of Bat^000000!",
                                "^FF00005 Scorpion Tail^000000!!",
                                "^FF00005 Yoyo Tail^000000!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args![
                                "^FF00005 Monster's Feed^000000!",
                                "^FF00005 Fluff^000000!!",
                                "^FF00005 Clover^000000!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args![
                                "^FF00005 Feather of Birds^000000!",
                                "^FF00005 Talon^000000!!",
                                "^FF00005 Spawn^000000!!"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["Don't even think about coming back until you've got all those or I'll kill you where you stand."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["What the F$@k? Did you just say I'm annoying you? Shut up, you ungrateful prick!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["I just added ^FF000010 Raccoon Leaf^000000 to the list. You better get it!"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["F%$#*&#@$%#@$@#$%@$%&*k! Haven't you ever thought about how hard it would be to be an NPC!?!"],
                        )?;
                        ctx.call(
                            Function::MapAnnounce,
                            vec![
                                Val::from("in_rogue"),
                                Val::from("F%$#*&#@$%#@$@#$%@$%&*k! Haven't you ever thought about how hard it would be to be an NPC!?!"),
                                ctx.constant("BC_MAP")?,
                            ],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    if ctx.var("rogue_q").get()?.number()? < 2 {
                        ctx.lines_as("Mr. Smith", args!["Three thousand, two hundred seventy two. Three thousand, two hundred seventy three. Three thousand, two hundred seventy four..."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Mr. Smith",
                            args!["Uhh...", "Headache...", "This is too much", "zeny to count."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["Uh...?", "What are you doing here? If you're going to talk about the job change, you need to speak to the other guy first."])?;
                        ctx.next()?;
                        ctx.lines_as("Mr. Smith", args!["...Shit!", "I lost count!"])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    } else {
                        if ctx.var("rogue_q").get()?.number()? > 2 {
                            if ctx.var("rogue_q").get()? == 3 {
                                mr_smith_rg_run(
                                    ctx,
                                    MrSmithRgStep::SCheckItems,
                                    vec![
                                        Val::from(510),
                                        Val::from(6),
                                        Val::from(932),
                                        Val::from(10),
                                        Val::from(957),
                                        Val::from(10),
                                        Val::from(958),
                                        Val::from(10),
                                    ],
                                )?;
                            } else {
                                if ctx.var("rogue_q").get()? == 4 {
                                    mr_smith_rg_run(
                                        ctx,
                                        MrSmithRgStep::SCheckItems,
                                        vec![
                                            Val::from(511),
                                            Val::from(10),
                                            Val::from(910),
                                            Val::from(10),
                                            Val::from(926),
                                            Val::from(10),
                                            Val::from(964),
                                            Val::from(10),
                                        ],
                                    )?;
                                } else {
                                    if ctx.var("rogue_q").get()? == 5 {
                                        mr_smith_rg_run(
                                            ctx,
                                            MrSmithRgStep::SCheckItems,
                                            vec![
                                                Val::from(508),
                                                Val::from(10),
                                                Val::from(948),
                                                Val::from(10),
                                                Val::from(935),
                                                Val::from(10),
                                                Val::from(940),
                                                Val::from(10),
                                            ],
                                        )?;
                                    } else {
                                        if ctx.var("rogue_q").get()? == 6 {
                                            if (((((((((((((((((ctx.var("Zeny").get()?.number()? > 9999
                                                && ctx.call(Function::CountItem, vec![Val::from(915)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(713)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(1002)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(953)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(507)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(919)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(715)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(913)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(904)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(942)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(528)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(914)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(705)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(916)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(917)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(908)])?.number()? > 4)
                                                && ctx.call(Function::CountItem, vec![Val::from(945)])?.number()? > 4)
                                            {
                                                ctx.lines_as("Mr. Smith", args!["Ummm...let's see..."])?;
                                                ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                                                ctx.call(Function::DelItem, vec![Val::from(915), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(713), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(1002), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(953), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(507), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(919), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(715), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(913), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(904), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(942), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(528), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(914), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(705), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(916), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(917), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(908), Val::from(5)])?;
                                                ctx.call(Function::DelItem, vec![Val::from(945), Val::from(5)])?;
                                                ctx.var("rogue_q").set(Val::from(8))?;
                                                ctx.next()?;
                                                ctx.lines_as("Mr. Smith", args!["Wow, you've brought each and every single thing I asked you to. Good work... I salute you."])?;
                                                ctx.next()?;
                                                ctx.lines(args!["^CCCCCC- Middle Finger -^000000'", "*Grins*"])?;
                                                ctx.var("rogue_q").set(Val::from(8))?;
                                                ctx.call(Function::ChangeQuest, vec![Val::from(2021), Val::from(2025)])?;
                                                ctx.next()?;
                                                ctx.lines_as("Mr. Smith", args!["Since you showed such great effort, I'm going to write a recommendation letter for you. I usually don't do that, you know."])?;
                                                ctx.next()?;
                                                ctx.lines_as("Mr. Smith", args!["But I'm sure you'll be a great asset to the Rogue Guild. Hmm, I don't have a blank piece of paper right now, so take this instead..."])?;
                                                ctx.call(Function::GetItem, vec![Val::from(1097), Val::from(1)])?;
                                                ctx.next()?;
                                                ctx.lines_as("Mr. Smith", args!["*Sigh...*", "I know, I know. I'm supposed to control myself in the work place. Getting enraged is a bad habit..."])?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Mr. Smith",
                                                    args![
                                                        "*Mumble mumble...*",
                                                        "How was I... *Mumble...*",
                                                        "How did... I remember...",
                                                        "*Sigh* It was all because of my bad temper!"
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as("Mr. Smith", args!["Wah....!!!"])?;
                                                ctx.next()?;
                                                ctx.lines(args!["^3355FFIt might be a better", "idea to come back later.^000000"])?;
                                                ctx.close_window()?;
                                                return Err(Stop::End);
                                            }
                                            ctx.lines_as(
                                                "Mr. Smith",
                                                args![
                                                    "Listen this time!",
                                                    "Application fee:",
                                                    "^FF000010000 zeny^000000,",
                                                    "^FF00005 Crysalis^000000!",
                                                    "^FF00005 Empty Bottle^000000!",
                                                    "^FF00005 Iron Ore^000000!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Mr. Smith",
                                                args![
                                                    "^FF00005 Stone Heart^000000!!",
                                                    "^FF00005 Red Herb^000000!",
                                                    "^FF00005 Animal Skin^000000!!",
                                                    "^FF00005 Yellow Gemstone^000000!!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Mr. Smith",
                                                args![
                                                    "^FF00005 Tooth of Bat^000000!",
                                                    "^FF00005 Scorpion Tail^000000!!",
                                                    "^FF00005 Yoyo Tail^000000!!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Mr. Smith",
                                                args![
                                                    "^FF00005 Monster's Feed^000000!",
                                                    "^FF00005 Fluff^000000!!",
                                                    "^FF00005 Clover^000000!!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as(
                                                "Mr. Smith",
                                                args![
                                                    "^FF00005 Feather of Birds^000000!",
                                                    "^FF00005 Talon^000000!!",
                                                    "^FF00005 Spawn^000000!!",
                                                    "^FF000010 Raccoon Leaf^000000!!"
                                                ],
                                            )?;
                                            ctx.next()?;
                                            ctx.lines_as("Mr. Smith", args!["Don't even think", "of coming back", "without them!"])?;
                                            ctx.close_window()?;
                                            return Err(Stop::End);
                                        } else {
                                            if ctx.var("rogue_q").get()? == 7 {
                                                ctx.lines_as("Mr. Smith", args!["Let me see...", "Who would should", "I send you to...?"])?;
                                                let subject1 = ctx.call(Function::Rand, vec![Val::from(1), Val::from(3)])?;
                                                if subject1 == 1 {
                                                    ctx.var("rogue_q").set(Val::from(9))?;
                                                    if ctx.call(Function::CheckQuest, vec![Val::from(2018)])? != -1 {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2018), Val::from(2022)])?;
                                                    } else if ctx.call(Function::CheckQuest, vec![Val::from(2019)])? != -1 {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2019), Val::from(2022)])?;
                                                    } else {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2020), Val::from(2022)])?;
                                                    }
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Right! I know", "just the guy~!"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Go visit Aragham Junior who lives South of the Sandarman Fortress. That area is located one field east from here."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["He's a pretty nice guy, you know. He works hard and is really good at bill collecting."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Before he joined the Rogue Guild, people have been trying to kill him for something his father did in the past. So, he became a runaway."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Well anyway, that's why he's been with us. We've been helping him hide from his enemies."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Ah, you might want to remember the password if you want to meet him. He doesn't let anybody in his house without the password."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args!["The password is ^0000FFAragham never hoarded upgrade items^000000."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Well, I will wish you luck. His place isn't that far from here, so come back as soon as possible. Being swift... That is the spirit of the Rogue."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if subject1 == 2 {
                                                    ctx.var("rogue_q").set(Val::from(10))?;
                                                    if ctx.call(Function::CheckQuest, vec![Val::from(2018)])? != -1 {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2018), Val::from(2023)])?;
                                                    } else if ctx.call(Function::CheckQuest, vec![Val::from(2019)])? != -1 {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2019), Val::from(2023)])?;
                                                    } else {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2020), Val::from(2023)])?;
                                                    }
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args![
                                                            "Hmm...",
                                                            "This guy might be",
                                                            "good for you, but...",
                                                            "He's a little dangerous."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["I want you to meet Antonio Junior, son of Antonio the first. For some reason people have been trying to kill him because of something his father did in the past."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["He was brought up in Payon, but he's staying in an empty house near the Kokomo beach at the moment."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["I've heard lately that he's been complaining a lot about the noise outside of his house, and he fears an assassination attempt. Anyway..."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["He's kind of tense, so he throws a dagger at anyone who approaches his house. He has a violent personality."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["However, he does have magnificent business skills. And he also loves gambling. Once you get to know him, he'll take care of your Rogue training really well."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Ah, you might want to remember the password to meet him in person. The password is ^0000FFAntonio doesn't enjoy destroying upgrade items^000000."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if subject1 == 3 {
                                                    ctx.var("rogue_q").set(Val::from(11))?;
                                                    if ctx.call(Function::CheckQuest, vec![Val::from(2018)])? != -1 {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2018), Val::from(2024)])?;
                                                    } else if ctx.call(Function::CheckQuest, vec![Val::from(2019)])? != -1 {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2019), Val::from(2024)])?;
                                                    } else {
                                                        ctx.call(Function::ChangeQuest, vec![Val::from(2020), Val::from(2024)])?;
                                                    }
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args![
                                                            "Hmm...",
                                                            "This guy might be",
                                                            "good for you, but...",
                                                            "He's a little dangerous."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args!["His name is", "Hollgrehenn Junior,", "a genius at manipulation."],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["However, because of something his father did long ago, people have been trying to kill him. Because of this, he is very high strung and will throw daggers at people he doesn't trust."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Our leader has been able to get him to join our guild, and his brilliant mind has been an asset to us. Once you get to know him, he'll take care of your Rogue training really well."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Ah, you might want to remember the password to meet him in person. The password is ^0000FFMy father never hoarded upgrade items^000000."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            } else {
                                                if ctx.var("rogue_q").get()? == 8 {
                                                    ctx.lines_as("Mr. Smith", args!["Alright... Now that I've calmed down, I can inform you of your next destination. *Whew*"])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Go and find Hermanthorn Junior, who is living near the ^0000FFthe checkpoint of Paros Lighthouse^000000, which is at the border between Morocc and Comodo."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Ah...almost forgot, keep in mind not to mention anything about upgrading items. This is very important."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rogue_q").get()? == 9 {
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args!["What...?", "Did you just", "say that you", "forgot where to go?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Head one field East and enter the building that is South of the Sandarman Fortress to meet Aragham Junior."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args!["The password is ^0000FFAragham never hoarded upgrade items^000000."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rogue_q").get()? == 10 {
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args!["What...?", "Did you just", "say that you", "forgot where to go?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args![
                                                            "Go to the building",
                                                            "at Kokomo Beach,",
                                                            "which is on the way",
                                                            "to Comodo, to meet",
                                                            "Antonio Junior."
                                                        ],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["The password is ^0000FF'Antonio doesn't enjoy destroying upgrade items'^000000."])?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rogue_q").get()? == 11 {
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args!["What...?", "Did you just", "say that you", "forgot where to go?"],
                                                    )?;
                                                    ctx.next()?;
                                                    ctx.lines_as("Mr. Smith", args!["Go to the field South of Sandarman Fortress, which is on the way to Morocc from here, to meet Hollgrehenn Junior."])?;
                                                    ctx.next()?;
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args!["The password is ^0000FFMy father never hoarded upgrade items^000000."],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                } else if ctx.var("rogue_q").get()?.number()? > 11 {
                                                    ctx.lines_as(
                                                        "Mr. Smith",
                                                        args![
                                                            "Hmmm...?",
                                                            "Don't you have",
                                                            "to go somewhere",
                                                            "else to complete",
                                                            "your Rogue training?"
                                                        ],
                                                    )?;
                                                    ctx.close_window()?;
                                                    return Err(Stop::End);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                step = MrSmithRgStep::SReq;
                continue 'machine;
            }
            MrSmithRgStep::SReq => {
                ctx.lines_as(
                    "Mr. Smith",
                    args!["First, the", "application fee:", "^FF000010,000 zeny^000000."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Smith",
                    args![
                        "We also need",
                        "you to bring",
                        ((Val::from("^FF0000") + runtime::arg(&args, 0, Val::from(0))) + Val::from("^000000,")),
                        ((Val::from("^FF0000") + runtime::arg(&args, 1, Val::from(0))) + Val::from("^000000,")),
                        ((Val::from("^FF0000") + runtime::arg(&args, 2, Val::from(0))) + Val::from("^000000 and")),
                        ((Val::from("^FF0000") + runtime::arg(&args, 3, Val::from(0))) + Val::from("^000000."))
                    ],
                )?;
                l_var = runtime::arg(&args, 4, Val::from(0));
                ctx.var("rogue_q").set(l_var.clone())?;
                if l_var.clone() == 3 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(2017), Val::from(2018)])?;
                } else if l_var.clone() == 4 {
                    ctx.call(Function::ChangeQuest, vec![Val::from(2017), Val::from(2019)])?;
                } else {
                    ctx.call(Function::ChangeQuest, vec![Val::from(2017), Val::from(2020)])?;
                }
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Smith",
                    args![
                        "Hmm...?",
                        "What was that?",
                        "Did you just say that",
                        "you're willing to donate",
                        "more for the guild?"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Smith",
                    args!["That sounds sweet,", "I appreciate that.", "But come back when", "you're ready."],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            MrSmithRgStep::SCheckItems => {
                let base = Val::from(0).number()?;
                runtime::local_set(
                    &mut l_item_req,
                    &Val::from(base + 0),
                    runtime::arg(&args, 0, Val::from(0)),
                    false,
                );
                runtime::local_set(
                    &mut l_item_req,
                    &Val::from(base + 1),
                    runtime::arg(&args, 2, Val::from(0)),
                    false,
                );
                runtime::local_set(
                    &mut l_item_req,
                    &Val::from(base + 2),
                    runtime::arg(&args, 4, Val::from(0)),
                    false,
                );
                runtime::local_set(
                    &mut l_item_req,
                    &Val::from(base + 3),
                    runtime::arg(&args, 6, Val::from(0)),
                    false,
                );
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_amount, &Val::from(base + 0), runtime::arg(&args, 1, Val::from(0)), false);
                runtime::local_set(&mut l_amount, &Val::from(base + 1), runtime::arg(&args, 3, Val::from(0)), false);
                runtime::local_set(&mut l_amount, &Val::from(base + 2), runtime::arg(&args, 5, Val::from(0)), false);
                runtime::local_set(&mut l_amount, &Val::from(base + 3), runtime::arg(&args, 7, Val::from(0)), false);
                if ((((ctx.var("Zeny").get()?.number()? > 9999
                    && runtime::op(
                        &ctx.call(Function::CountItem, vec![runtime::local_get(&l_item_req, &Val::from(0), false)])?,
                        ">=",
                        &runtime::local_get(&l_amount, &Val::from(0), false),
                    )?
                    .is_true())
                    && runtime::op(
                        &ctx.call(Function::CountItem, vec![runtime::local_get(&l_item_req, &Val::from(1), false)])?,
                        ">=",
                        &runtime::local_get(&l_amount, &Val::from(1), false),
                    )?
                    .is_true())
                    && runtime::op(
                        &ctx.call(Function::CountItem, vec![runtime::local_get(&l_item_req, &Val::from(2), false)])?,
                        ">=",
                        &runtime::local_get(&l_amount, &Val::from(2), false),
                    )?
                    .is_true())
                    && runtime::op(
                        &ctx.call(Function::CountItem, vec![runtime::local_get(&l_item_req, &Val::from(3), false)])?,
                        ">=",
                        &runtime::local_get(&l_amount, &Val::from(3), false),
                    )?
                    .is_true())
                {
                    ctx.lines_as(
                        "Mr. Smith",
                        args![
                            ((((((((((((((((Val::from("Okay, we've got the application fee, ^FF000010,000 zeny^000000, ")
                                + runtime::local_get(&l_amount, &Val::from(0), false))
                                + Val::from(" "))
                                + ctx.call(
                                    Function::GetItemName,
                                    vec![runtime::local_get(&l_item_req, &Val::from(0), false)]
                                )?)
                                + Val::from(", "))
                                + runtime::local_get(&l_amount, &Val::from(1), false))
                                + Val::from(" "))
                                + ctx.call(
                                    Function::GetItemName,
                                    vec![runtime::local_get(&l_item_req, &Val::from(1), false)]
                                )?)
                                + Val::from(", "))
                                + runtime::local_get(&l_amount, &Val::from(2), false))
                                + Val::from(" "))
                                + ctx.call(
                                    Function::GetItemName,
                                    vec![runtime::local_get(&l_item_req, &Val::from(2), false)]
                                )?)
                                + Val::from(" and "))
                                + runtime::local_get(&l_amount, &Val::from(3), false))
                                + Val::from(" "))
                                + ctx.call(
                                    Function::GetItemName,
                                    vec![runtime::local_get(&l_item_req, &Val::from(3), false)]
                                )?)
                                + Val::from("..."))
                        ],
                    )?;
                    ctx.var("Zeny").set((ctx.var("Zeny").get()?.try_sub(Val::from(10000))?))?;
                    ctx.call(
                        Function::DelItem,
                        vec![
                            runtime::local_get(&l_item_req, &Val::from(0), false),
                            runtime::local_get(&l_amount, &Val::from(0), false),
                        ],
                    )?;
                    ctx.call(
                        Function::DelItem,
                        vec![
                            runtime::local_get(&l_item_req, &Val::from(1), false),
                            runtime::local_get(&l_amount, &Val::from(1), false),
                        ],
                    )?;
                    ctx.call(
                        Function::DelItem,
                        vec![
                            runtime::local_get(&l_item_req, &Val::from(2), false),
                            runtime::local_get(&l_amount, &Val::from(2), false),
                        ],
                    )?;
                    ctx.call(
                        Function::DelItem,
                        vec![
                            runtime::local_get(&l_item_req, &Val::from(3), false),
                            runtime::local_get(&l_amount, &Val::from(3), false),
                        ],
                    )?;
                    ctx.var("rogue_q").set(Val::from(7))?;
                    ctx.next()?;
                    ctx.lines_as("Mr. Smith", args!["Great, great...", "I think you", "brought everything."])?;
                    ctx.var("rogue_q").set(Val::from(7))?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Mr. Smith",
                        args![
                            "Alright, wait just a moment while",
                            "I prepare these things. Let's see... Your next test is..."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                ctx.lines_as("Mr. Smith", args!["What the f$@k!? You didn't bring all the required items?! Are you telling me that you need to check the requirements again!?"])?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Smith",
                    args!["Now listen...!", "Bring ^FF000010,000 zeny^000000,", "and the following items..."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Mr. Smith",
                    args![
                        ((((Val::from("^FF0000 ") + runtime::local_get(&l_amount, &Val::from(0), false)) + Val::from(" "))
                            + ctx.call(
                                Function::GetItemName,
                                vec![runtime::local_get(&l_item_req, &Val::from(0), false)]
                            )?)
                            + Val::from("^000000,")),
                        ((((Val::from("^FF0000 ") + runtime::local_get(&l_amount, &Val::from(1), false)) + Val::from(" "))
                            + ctx.call(
                                Function::GetItemName,
                                vec![runtime::local_get(&l_item_req, &Val::from(1), false)]
                            )?)
                            + Val::from("^000000,")),
                        ((((Val::from("^FF0000 ") + runtime::local_get(&l_amount, &Val::from(2), false)) + Val::from(" "))
                            + ctx.call(
                                Function::GetItemName,
                                vec![runtime::local_get(&l_item_req, &Val::from(2), false)]
                            )?)
                            + Val::from("^000000,")),
                        ((((Val::from("^FF0000 ") + runtime::local_get(&l_amount, &Val::from(3), false)) + Val::from(" "))
                            + ctx.call(
                                Function::GetItemName,
                                vec![runtime::local_get(&l_item_req, &Val::from(3), false)]
                            )?)
                            + Val::from("^000000,")),
                        "You got it this time?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn mr_smith_rg(ctx: &Ctx) -> Script {
    mr_smith_rg_run(ctx, MrSmithRgStep::Start, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp1Step {
    Start,
    OnTouch,
}

fn warp_1_run(ctx: &Ctx, mut step: Warp1Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rogue_t = Val::from(0);
    'machine: loop {
        match step {
            Warp1Step::Start => {
                step = Warp1Step::OnTouch;
                continue 'machine;
            }
            Warp1Step::OnTouch => {
                ctx.lines_as("???", args!["Who's there?!", "Who would dare", "intrude my territory?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("My father:Aragham:Aragon:Legolas")])? {
                    1 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["My father"])?;
                    }
                    2 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Aragham"])?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    3 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Aragon"])?;
                    }
                    4 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Legolas"])?;
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("did not:didn't:never:ever")])? {
                    1 => {
                        ctx.mes("did not")?;
                    }
                    2 => {
                        ctx.mes("didn't")?;
                    }
                    3 => {
                        ctx.mes("never")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    4 => {
                        ctx.mes("ever")?;
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("hoard:hoarded:hide:took:take")])? {
                    1 => {
                        ctx.mes("hoard")?;
                    }
                    2 => {
                        ctx.mes("hoarded")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    3 => {
                        ctx.mes("hide")?;
                    }
                    4 => {
                        ctx.mes("took")?;
                    }
                    5 => {
                        ctx.mes("take")?;
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("upgrade items.:forging items.:refining item.:upgrade item.")])? {
                    1 => {
                        ctx.mes("upgrade items.")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    2 => {
                        ctx.mes("forging items.")?;
                    }
                    3 => {
                        ctx.mes("refining item.")?;
                    }
                    4 => {
                        ctx.mes("upgrade item.")?;
                    }
                    _ => {}
                }
                ctx.next()?;
                if l_rogue_t.clone().number()? > 30 {
                    ctx.lines(args!["^3355FF*Creeeeak*", "The door slowly opens.^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(246), Val::from(25)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("???", args!["What the...?", "Get lost!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn warp_1(ctx: &Ctx) -> Script {
    warp_1_run(ctx, Warp1Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_1_ontouch(ctx: &Ctx) -> Script {
    warp_1_run(ctx, Warp1Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp2Step {
    Start,
    OnTouch,
}

fn warp_2_run(ctx: &Ctx, mut step: Warp2Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rogue_t = Val::from(0);
    'machine: loop {
        match step {
            Warp2Step::Start => {
                step = Warp2Step::OnTouch;
                continue 'machine;
            }
            Warp2Step::OnTouch => {
                ctx.lines_as("???", args!["Who's there?!", "Who would dare", "intrude my territory?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("My father:Aragham:Aragon:Legolas")])? {
                    1 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["My father"])?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    2 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Aragham"])?;
                    }
                    3 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Aragon"])?;
                    }
                    4 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Legolas"])?;
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("did not:didn't:never:ever")])? {
                    1 => {
                        ctx.mes("did not")?;
                    }
                    2 => {
                        ctx.mes("didn't")?;
                    }
                    3 => {
                        ctx.mes("never")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    4 => {
                        ctx.mes("ever")?;
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("hoard:takes:hide:took:hoarded")])? {
                    1 => {
                        ctx.mes("hoard")?;
                    }
                    2 => {
                        ctx.mes("takes")?;
                    }
                    3 => {
                        ctx.mes("hide")?;
                    }
                    4 => {
                        ctx.mes("took")?;
                    }
                    5 => {
                        ctx.mes("hoarded")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("upgrade items.:forging items.:refining item.:upgrade item.")])? {
                    1 => {
                        ctx.mes("upgrade items.")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    2 => {
                        ctx.mes("forging items.")?;
                    }
                    3 => {
                        ctx.mes("refining item.")?;
                    }
                    4 => {
                        ctx.mes("upgrade item.")?;
                    }
                    _ => {}
                }
                ctx.next()?;
                if l_rogue_t.clone().number()? > 30 {
                    ctx.lines(args!["^3355FF*Creeeeak*", "The door slowly opens.^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(169), Val::from(34)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("???", args!["What the...?", "Get lost!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn warp_2(ctx: &Ctx) -> Script {
    warp_2_run(ctx, Warp2Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_2_ontouch(ctx: &Ctx) -> Script {
    warp_2_run(ctx, Warp2Step::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Warp3Step {
    Start,
    OnTouch,
}

fn warp_3_run(ctx: &Ctx, mut step: Warp3Step, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_rogue_t = Val::from(0);
    'machine: loop {
        match step {
            Warp3Step::Start => {
                step = Warp3Step::OnTouch;
                continue 'machine;
            }
            Warp3Step::OnTouch => {
                ctx.lines_as("???", args!["Who's there?!", "Who would dare", "intrude my territory?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Anntonio:Aragham:Antonio:Hollgrehenn")])? {
                    1 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Anntonio"])?;
                    }
                    2 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Aragham"])?;
                    }
                    3 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Antonio"])?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    4 => {
                        ctx.lines_as(ctx.call(Function::StrCharInfo, vec![Val::from(0)])?, args!["Hollgrehenn"])?;
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("enjoys:doesn't enjoy:likes:doesn't like")])? {
                    1 => {
                        ctx.mes("enjoys")?;
                    }
                    2 => {
                        ctx.mes("doesn't enjoy")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    3 => {
                        ctx.mes("likes")?;
                    }
                    4 => {
                        ctx.mes("doesn't like")?;
                    }
                    _ => {}
                }
                match runtime::select_values(ctx, &[Val::from("damaging:destroying:fixing:forging")])? {
                    1 => {
                        ctx.mes("damaging")?;
                    }
                    2 => {
                        ctx.mes("destroying")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    3 => {
                        ctx.mes("fixing")?;
                    }
                    4 => {
                        ctx.mes("forging")?;
                    }
                    _ => {}
                }
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "forging item.:refining items.:upgrade items.:refined items.:upgraded items.:forged items.",
                    )],
                )? {
                    1 => {
                        ctx.mes("forging item.")?;
                    }
                    2 => {
                        ctx.mes("refining items.")?;
                    }
                    3 => {
                        ctx.mes("upgrade items.")?;
                        l_rogue_t = (l_rogue_t.clone() + Val::from(10));
                    }
                    4 => {
                        ctx.mes("refined items.")?;
                    }
                    5 => {
                        ctx.mes("upgraded items.")?;
                    }
                    6 => {
                        ctx.mes("forged items.")?;
                    }
                    _ => {}
                }
                ctx.next()?;
                if l_rogue_t.clone().number()? > 30 {
                    ctx.lines(args!["^3355FF*Creeeeak*", "The door slowly opens.^000000"])?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(164), Val::from(106)])?;
                    return Err(Stop::End);
                } else {
                    ctx.lines_as("???", args![".....Get lost!"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn warp_3(ctx: &Ctx) -> Script {
    warp_3_run(ctx, Warp3Step::Start, Vec::new()).map(|_| ())
}

pub fn warp_3_ontouch(ctx: &Ctx) -> Script {
    warp_3_run(ctx, Warp3Step::OnTouch, Vec::new()).map(|_| ())
}

fn hermanthorn_jr_rg_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("rogue_q").get()? == 8 {
        ctx.lines_as(
            "HermanthornJr.",
            args![
                "I see...",
                "You must be from",
                "the Rogue guild.",
                "You must be one of the",
                "ones Mr. Smith wasn't",
                "too happy with..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("HermanthornJr.", args!["He threw a fit and you brought him all the items he asked for, didn't you? Well, I can see that you're still pretty naive. Hahaha~"])?;
        ctx.next()?;
        ctx.lines_as(
            "HermanthornJr.",
            args!["I suppose he suckered you into gathering those items, and then passed you on to me. Sad, really."],
        )?;
        ctx.next()?;
        ctx.lines_as("HermanthornJr.", args!["Well, since you were tortured by him, I'll try to be especially generous to you. My test for you will be simple, so simple."])?;
        ctx.next()?;
        ctx.lines_as(
            "HermanthornJr.",
            args!["All you have to do is go through an underground tunnel, and walk all the way back to the Rogue Guild."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "HermanthornJr.",
            args!["There is one thing I should tell you, though. You might want to be careful inside, alright?"],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "HermanthornJr.",
            args!["A bunch of pricks have been throwing Dead Branches and casting Hocus Pocus all over the place..."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "HermanthornJr.",
            args!["Well...", "Just make it back to the Rogue Guild alive. That's all you have to do!"],
        )?;
        ctx.var("rogue_q").set(Val::from(12))?;
        ctx.call(Function::ChangeQuest, vec![Val::from(2025), Val::from(2026)])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else if ctx.var("rogue_q").get()? == 12 {
        ctx.lines_as("HermanthornJr.", args!["Oh right. This is really important. You need a password to enter the tunnel. To unlock the door, the four number combination is ^0000FF3019^000000."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "HermanthornJr.",
            args!["Huh...?", "What the hell", "are you doing here.", "Scram, why don't you?"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
}

pub fn hermanthorn_jr_rg(ctx: &Ctx) -> Script {
    hermanthorn_jr_rg_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum HeToRogueRgStep {
    Start,
    OnTouch,
}

fn he_to_rogue_rg_run(ctx: &Ctx, mut step: HeToRogueRgStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_input = Val::from(0);
    'machine: loop {
        match step {
            HeToRogueRgStep::Start => {
                step = HeToRogueRgStep::OnTouch;
                continue 'machine;
            }
            HeToRogueRgStep::OnTouch => {
                ctx.mes("^3355FFThe door is locked. You'll need to enter the four number combination to open it.^000000")?;
                ctx.next()?;
                let (input, status) = runtime::input_number(ctx, None, None)?;
                l_input = input;
                if (l_input.clone().number()? < 1 || l_input.clone().number()? > 10000) {
                    if ctx.var("rogue_q").get()? == 12 {
                        ctx.mes("^3355FFIt didn't work. Please re-enter the four number combination.^000000")?;
                    } else {
                        ctx.mes("^3355FFPlease enter a combination of four numbers.^000000")?;
                    }
                    ctx.close_window()?;
                    return Err(Stop::End);
                } else if l_input.clone() == 3019 {
                    if ctx.var("rogue_q").get()? == 12 {
                        ctx.lines(args!["^3355FFThe door", "has opened.^000000"])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("in_rogue"), Val::from(10), Val::from(21)])?;
                        ctx.var("rogue_q").set(Val::from(12))?;
                        return Err(Stop::End);
                    } else {
                        ctx.lines_as(
                            "HermanthornJr.",
                            args!["Well...", "Didn't I tell you", "that I changed the", "password? *Wink Wink*"],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                } else {
                    ctx.lines(args!["^3355FFThe door", "is still locked.^000000"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        }
    }
}

pub fn he_to_rogue_rg(ctx: &Ctx) -> Script {
    he_to_rogue_rg_run(ctx, HeToRogueRgStep::Start, Vec::new()).map(|_| ())
}

pub fn he_to_rogue_rg_ontouch(ctx: &Ctx) -> Script {
    he_to_rogue_rg_run(ctx, HeToRogueRgStep::OnTouch, Vec::new()).map(|_| ())
}
