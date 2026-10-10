use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn gunpowder_expert_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("ch_make").get()? == 0 {
        runtime::party_members(ctx, ctx.call(Function::GetCharacterId, vec![Val::from(1)])?, Val::from(0))?;
        ctx.var("@partymember").set(ctx.var("$@partymembercount").get()?)?;
        if !(ctx.var("ql_revol").get()?.is_true()) {
            ctx.lines_as("Hao Chenryu", args!["Who...", "Are you?"])?;
            ctx.next()?;
            ctx.lines_as(
                "Hao Chenryu",
                args![
                    "I only trust my comrades in the cause for Luoyang, so leave me alone. Death before oppression! Freedom for Luoyang!"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ctx.var("@partymember").get()? != 8 {
            ctx.lines_as(
                "Hao Chenryu",
                args![
                    "Mao told me that eight Midgardians",
                    "would be coming.",
                    "But what happened?",
                    "I'm confused..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Hao Chenryu", args!["Mao's plans are flawless,", "so I'm sticking to the scenario he's drawn up. You're either improvising needlessly, or you're not really my comrade..."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else if ((ctx.call(Function::CountItem, vec![Val::from(7068)])?.is_true()
            && ctx.call(Function::CountItem, vec![Val::from(7096)])?.is_true())
            && ctx.call(Function::CountItem, vec![Val::from(7004)])?.is_true())
        {
            ctx.lines_as(
                "Hao Chenryu",
                args![
                    "Ah, I see that Mao",
                    "has sent you. I'm sorry",
                    "for the trouble you've",
                    "gone through to smuggle",
                    "the chemicals to me, comrade."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Hao Chenryu",
                args!["Please give me", "a minute so that", "I may quickly make", "the gunpowder."],
            )?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Make Gunpowder.:Cancel.")])? {
                1 => {
                    ctx.call(Function::DelItem, vec![Val::from(7068), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7096), Val::from(1)])?;
                    ctx.call(Function::DelItem, vec![Val::from(7004), Val::from(1)])?;
                    ctx.lines_as("Hao Chenryu", args!["I don't want you to be caught if the soldiers inspect your goods. It will be better if one of you carry a Cart and hide the gunpowder in there."])?;
                    ctx.var("ch_make").set(Val::from(1))?;
                    ctx.call(Function::GetItem, vec![Val::from(7204), Val::from(1)])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Hao Chenryu", args!["Is...", "Something wrong?!"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Hao Chenryu",
                        args![
                            "I guess we must",
                            "be extra careful,",
                            "as there is much",
                            "surveillance in",
                            "this place."
                        ],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else {
            ctx.lines_as("Hao Chenryu", args!["Hmmm...", "It seems like", "you're one of us."])?;
            ctx.next()?;
            ctx.lines_as(
                "Hao Chenryu",
                args![
                    "If you came for the",
                    "gunpowder, you should",
                    "get some chemcials",
                    "from your party leader."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else if ctx.call(Function::CountItem, vec![Val::from(7204)])?.is_true() {
        ctx.lines_as(
            "Hao Chenryu",
            args![
                "Be careful!",
                "It's easy for",
                "gunpowder to be",
                "found during the",
                "inspection process!",
                "Hide it well...!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        ctx.lines_as(
            "Hao Chenryu",
            args!["I hope you bring the gunpowder to Mao safely. The next time you see him, please give him my thanks."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    Ok(Val::from(0))
}

pub fn gunpowder_expert(ctx: &Ctx) -> Script {
    gunpowder_expert_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.call(Function::CountItem, vec![Val::from(7204)])?.is_true() {
        ctx.lines_as("Soldier", args!["Hold on there!"])?;
        ctx.next()?;
        ctx.lines_as(
            "Soldier",
            args![
                "You're not allowed to bring any gunpowder out of here. Now go",
                "and put it back, Midgardian! Move!"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Soldier",
        args![
            "It's strange to see outsiders in this kind of place. We don't have any business with you,",
            "so move along."
        ],
    )?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("lou_in01"), Val::from(82), Val::from(141)])?;
    return Err(Stop::End);
}

pub fn soldier_1(ctx: &Ctx) -> Script {
    soldier_1_body(ctx, Vec::new()).map(|_| ())
}

fn soldier_2_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ((ctx.call(Function::CountItem, vec![Val::from(7068)])?.is_true()
        || ctx.call(Function::CountItem, vec![Val::from(7096)])?.is_true())
        || ctx.call(Function::CountItem, vec![Val::from(7004)])?.is_true())
    {
        ctx.lines_as(
            "Soldier",
            args![
                "I am sorry, but it is prohibited to enter with some of the products",
                "you are carrying. I cannot let you in."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Soldier",
        args!["There is nothing much, but if you wish to look around this room, please be my guest."],
    )?;
    ctx.close_window()?;
    ctx.call(Function::Warp, vec![Val::from("lou_in01"), Val::from(47), Val::from(141)])?;
    return Err(Stop::End);
}

pub fn soldier_2(ctx: &Ctx) -> Script {
    soldier_2_body(ctx, Vec::new()).map(|_| ())
}
