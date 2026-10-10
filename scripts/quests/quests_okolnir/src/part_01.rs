use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn guide_gq_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_defence = Val::from(0);
    let mut l_economy = Val::from(0);
    let mut l_gid = Val::from(0);
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_gid = ctx.call(
        Function::GetCastleData,
        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(1)],
    )?;
    if ctx
        .call(Function::GetCharacterId, vec![Val::from(2)])?
        .loosely_equals(&l_gid.clone())
    {
        if runtime::getd(
            ctx,
            &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
            &[
                (".@defence", runtime::Local::Scalar(&l_defence)),
                (".@economy", runtime::Local::Scalar(&l_economy)),
                (".@gid", runtime::Local::Scalar(&l_gid)),
                (".@sub$", runtime::Local::Scalar(&l_sub_s)),
            ],
        )? == 0
        {
            ctx.lines_as(
                "Guide",
                args!["This castle has a hidden secret.", "That is the ^4d4dff'Okolnir'^000000."],
            )?;
            ctx.next()?;
            'b1: {
                let subject1 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("About Okolnir.:Go to Okolnir.:Cancel.")],
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
                        "Guide",
                        args![
                            "Okolnir is a kind of virtual realm...",
                            "I don't know how Okolnir exists, but I guess only Valkyrie knows."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "As you know this is a place to test the adventurers made by Valkyrie...",
                            "...you know the qualifications to enter Okolnir."
                        ],
                    )?;
                    ctx.next()?;
                    let choice = runtime::select_values(ctx, &[Val::from("Qualifications?")])?;
                    ctx.var("@menu").set(choice)?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "Yes, Valkyrie definitely prefers strong adventurers.",
                            "Only the qualified can enter Okolnir and Valhalla."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "It only opens when everyone comes together to work it out.",
                            "The key is in the castle."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Guide", args!["If a castle's ^4d4dffeconomy is over 65 and defense also over 30^000000, this will be acceptable to access Okolnir."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Guide",
                        args![
                            "And, if you pass all of the tests given by Valkyrie in Okolnir!",
                            "You will also receive a mysterious gift."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Guide", args!["Would you like to try to enter here?"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                    matched1 = true;
                }
                if matched1 {
                    l_defence = ctx.call(
                        Function::GetCastleData,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(3)],
                    )?;
                    l_economy = ctx.call(
                        Function::GetCastleData,
                        vec![ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, Val::from(2)],
                    )?;
                    if (l_economy.clone().number()? > 64 && l_defence.clone().number()? > 29) {
                        ctx.lines_as(
                            "Guide",
                            args![
                                "Great! Economy and Defense are OK.",
                                "You can enter Okolnir now....",
                                "Do you want to go there?"
                            ],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("Sure let's go there.:No.")])? {
                            1 => {
                                if ctx.call(Function::CountItem, vec![Val::from(7839)])?.is_true() {
                                    ctx.call(
                                        Function::DelItem,
                                        vec![Val::from(7839), ctx.call(Function::CountItem, vec![Val::from(7839)])?],
                                    )?;
                                }
                                ctx.lines_as("Guide", args!["Ok......", "Please follow me..."])?;
                                ctx.close_window()?;
                                ctx.call(
                                    Function::Warp,
                                    vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(346), Val::from(32)],
                                )?;
                                return Err(Stop::End);
                            }
                            2 => {
                                ctx.lines_as(
                                    "Guide",
                                    args![
                                        "You can try this anytime in the future...",
                                        "If you are ready to protect this castle."
                                    ],
                                )?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    } else {
                        ctx.lines_as(
                            "Guide",
                            args!["You are not qualified yet.", "Please develop your castle more..."],
                        )?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                }
                if !matched1 && subject1.loosely_equals(&Val::from(3)) {
                    matched1 = true;
                }
                if matched1 {
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
            }
        } else if runtime::getd(
            ctx,
            &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
            &[
                (".@defence", runtime::Local::Scalar(&l_defence)),
                (".@economy", runtime::Local::Scalar(&l_economy)),
                (".@gid", runtime::Local::Scalar(&l_gid)),
                (".@sub$", runtime::Local::Scalar(&l_sub_s)),
            ],
        )? == 1
        {
            ctx.lines_as("Guide", args!["... OK...", "Good luck."])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Enter now.:No.")])? {
                1 => {
                    if ctx.call(Function::CountItem, vec![Val::from(7839)])?.number()? > 0 {
                        ctx.call(
                            Function::DelItem,
                            vec![Val::from(7839), ctx.call(Function::CountItem, vec![Val::from(7839)])?],
                        )?;
                    }
                    ctx.lines_as("Guide", args!["Hope you get everything you want..."])?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::Warp,
                        vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(346), Val::from(32)],
                    )?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Guide", args!["Really?", "Sorry to hear that."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        } else if runtime::getd(
            ctx,
            &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
            &[
                (".@defence", runtime::Local::Scalar(&l_defence)),
                (".@economy", runtime::Local::Scalar(&l_economy)),
                (".@gid", runtime::Local::Scalar(&l_gid)),
                (".@sub$", runtime::Local::Scalar(&l_sub_s)),
            ],
        )? == 2
        {
            ctx.lines_as(
                "Guide",
                args!["Building Okolnir needs quite a long time.", "....even though it's only virtual..."],
            )?;
            ctx.next()?;
            ctx.lines_as("Guide", args!["It takes about 12 - 13 hours to create the virtual realm."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            ctx.lines_as("Guide", args!["You'll have to wait."])?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Guide",
            args![
                "... I've never seen you before.",
                "You are strangers here. You'd better get out of here right now."
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    return Err(Stop::End);
}

pub fn guide_gq_main(ctx: &Ctx) -> Script {
    guide_gq_main_body(ctx, Vec::new()).map(|_| ())
}

fn guide_gq_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::EnableNpc, vec![(Val::from("Guide#gq_") + l_sub_s.clone())])?;
    if runtime::getd(
        ctx,
        &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
        &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )? == 1
    {
        runtime::setd(
            ctx,
            &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
            Val::from(0),
            &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
        )?;
    }
    return Err(Stop::End);
}

pub fn guide_gq_main_oninit(ctx: &Ctx) -> Script {
    guide_gq_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_gq_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_check = Val::from(0);
    let mut l_gid = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_saram = Val::from(0);
    let mut l_sub_s = Val::from("");
    let mut l_t_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    l_t_s = ((if runtime::compare(&ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?, &Val::from("aru")).is_true() {
        Val::from("arug_cas0")
    } else {
        Val::from("schg_cas0")
    }) + runtime::charat(
        &ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
        &(runtime::strlen(&ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?).try_sub(Val::from(1))?),
    )?);
    l_gid = ctx.call(Function::GetCastleData, vec![l_t_s.clone(), Val::from(1)])?;
    if ctx
        .call(Function::GetCharacterId, vec![Val::from(2)])?
        .loosely_equals(&l_gid.clone())
    {
        ctx.call(Function::Cutin, vec![Val::from("wish_maiden31"), Val::from(1)])?;
        if ctx
            .call(
                Function::GetGuildInfo,
                vec![ctx.call(Function::GetCharacterId, vec![Val::from(2)])?, Val::from(2)],
            )?
            .loosely_equals(&Val::from(1))
        {
            ctx.lines_as(
                "Wish Maiden",
                args![
                    "I am... Wish maiden.",
                    "Mourning in this virtual realm, Okolnir.",
                    "On behalf of the humanbeings who defeated God here."
                ],
            )?;
            ctx.next()?;
            if runtime::compare(&l_sub_s.clone(), &Val::from("aru")).is_true() {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(7835), false);
                runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(7836), false);
                runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(7837), false);
                runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 6), Val::from(7838), false);
                runtime::local_set(&mut l_n, &Val::from(base + 7), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 8), Val::from(2513), false);
                runtime::local_set(&mut l_n, &Val::from(base + 9), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 10), Val::from(7291), false);
                runtime::local_set(&mut l_n, &Val::from(base + 11), Val::from(10), false);
                runtime::local_set(&mut l_n, &Val::from(base + 12), Val::from(7293), false);
                runtime::local_set(&mut l_n, &Val::from(base + 13), Val::from(10), false);
                runtime::local_set(&mut l_n, &Val::from(base + 14), Val::from(7063), false);
                runtime::local_set(&mut l_n, &Val::from(base + 15), Val::from(100), false);
                runtime::local_set(&mut l_n, &Val::from(base + 16), Val::from(985), false);
                runtime::local_set(&mut l_n, &Val::from(base + 17), Val::from(20), false);
            } else {
                let base = Val::from(0).number()?;
                runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(7830), false);
                runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(7831), false);
                runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(7832), false);
                runtime::local_set(&mut l_n, &Val::from(base + 5), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 6), Val::from(7833), false);
                runtime::local_set(&mut l_n, &Val::from(base + 7), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 8), Val::from(7834), false);
                runtime::local_set(&mut l_n, &Val::from(base + 9), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 10), Val::from(2357), false);
                runtime::local_set(&mut l_n, &Val::from(base + 11), Val::from(1), false);
                runtime::local_set(&mut l_n, &Val::from(base + 12), Val::from(7510), false);
                runtime::local_set(&mut l_n, &Val::from(base + 13), Val::from(100), false);
                runtime::local_set(&mut l_n, &Val::from(base + 14), Val::from(969), false);
                runtime::local_set(&mut l_n, &Val::from(base + 15), Val::from(10), false);
                runtime::local_set(&mut l_n, &Val::from(base + 16), Val::from(985), false);
                runtime::local_set(&mut l_n, &Val::from(base + 17), Val::from(20), false);
            }
            l_i = Val::from(0);
            'l1: loop {
                if !(runtime::op(&l_i.clone(), "<", &Val::from(l_n.len() as i32))?.is_true()) {
                    break 'l1;
                }
                'b1: {
                    if runtime::op(
                        &ctx.call(Function::CountItem, vec![runtime::local_get(&l_n, &l_i.clone(), false)])?,
                        ">=",
                        &runtime::local_get(&l_n, &(l_i.clone() + Val::from(1)), false),
                    )?
                    .is_true()
                    {
                        l_check = (l_check.clone() + Val::from(1));
                    }
                }
                l_i = (l_i.clone() + Val::from(2));
            }
            if l_check.clone().number()? >= 9 {
                ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(1)])?;
                ctx.lines_as(
                    "Wish Maiden",
                    args!["Are you ready to endure the trials to get the Goddess' glory?"],
                )?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes, I am:Sorry, I'll try later")])? {
                    1 => {
                        ctx.call(Function::Cutin, vec![Val::from("wish_maiden12"), Val::from(1)])?;
                        ctx.lines_as(
                            "Wish Maiden",
                            args!["I will test whether or not you deserve the Goddess shine...", "Isn't it simple?"],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Wish Maiden",
                            args![
                                "Okolnir is a virtual place.",
                                "There is no room for error there.",
                                "^ff0000You only have one hour.^000000"
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("wish_maiden31"), Val::from(1)])?;
                        ctx.lines_as(
                            "Wish Maiden",
                            args![
                                "If you have not finished in that time, Okolnir will be destroyed, and I will go to rest.",
                                "You will have to wait again..."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Wish Maiden",
                            args![
                                "Are you ready to go through?",
                                "^4d4dffYou need to have 16 to 20 members present^000000."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(1)])?;
                        ctx.lines_as(
                            "Wish Maiden",
                            args!["I will open the gate of Okolnir if your members are ready."],
                        )?;
                        ctx.next()?;
                        match runtime::select_values(ctx, &[Val::from("We are ready.:We need more time.")])? {
                            1 => {
                                l_saram = ctx.call(Function::GetMapUsers, vec![(Val::from("que_q") + l_sub_s.clone())])?;
                                if (l_saram.clone().number()? > 15 && l_saram.clone().number()? < 21) {
                                    ctx.call(Function::Cutin, vec![Val::from("wish_maiden12"), Val::from(1)])?;
                                    ctx.lines_as(
                                        "Wish Maiden",
                                        args![
                                            "Now I will open the gate of Okolnir where I am.",
                                            "I will wait for you on the top of Okolnir..."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Wish Maiden",
                                        args![
                                            "I hope that you can complete all of the trials before the virtual Okolnir is destroyed...",
                                            "Good luck."
                                        ],
                                    )?;
                                    ctx.call(
                                        Function::MapAnnounce,
                                        vec![
                                            (Val::from("que_q") + l_sub_s.clone()),
                                            Val::from("Wish Maiden: The gate of Okolnir is open! Don't forget you only have one hour."),
                                            ctx.constant("BC_MAP")?,
                                            Val::from("0x00ff00"),
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    runtime::setd(
                                        ctx,
                                        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_pcc")),
                                        l_saram.clone(),
                                        &mut [
                                            (".@check", runtime::LocalMut::Scalar(&mut l_check)),
                                            (".@gid", runtime::LocalMut::Scalar(&mut l_gid)),
                                            (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                            (".@n", runtime::LocalMut::Array(&mut l_n)),
                                            (".@saram", runtime::LocalMut::Scalar(&mut l_saram)),
                                            (".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s)),
                                            (".@t$", runtime::LocalMut::Scalar(&mut l_t_s)),
                                        ],
                                    )?;
                                    runtime::setd(
                                        ctx,
                                        &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
                                        Val::from(1),
                                        &mut [
                                            (".@check", runtime::LocalMut::Scalar(&mut l_check)),
                                            (".@gid", runtime::LocalMut::Scalar(&mut l_gid)),
                                            (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                                            (".@n", runtime::LocalMut::Array(&mut l_n)),
                                            (".@saram", runtime::LocalMut::Scalar(&mut l_saram)),
                                            (".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s)),
                                            (".@t$", runtime::LocalMut::Scalar(&mut l_t_s)),
                                        ],
                                    )?;
                                    ctx.call(
                                        Function::DoNpcEvent,
                                        vec![((Val::from("#okolnir_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
                                    )?;
                                    ctx.call(Function::DisableNpc, vec![(Val::from("Wish Maiden#gq_") + l_sub_s.clone())])?;
                                    ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(255)])?;
                                    ctx.call(
                                        Function::Announce,
                                        vec![
                                            ((((Val::from("[") + ctx.call(Function::StrCharInfo, vec![Val::from(0)])?)
                                                + Val::from("], of the guild ["))
                                                + ctx.call(Function::GetGuildInfo, vec![l_gid.clone(), Val::from(0)])?)
                                                + Val::from("] has opened the gates to the realm of Okolnir.")),
                                            ctx.constant("BC_ALL")?,
                                            Val::from("0x70dbdb"),
                                        ],
                                    )?;
                                    return Err(Stop::End);
                                } else {
                                    ctx.call(Function::Cutin, vec![Val::from("wish_maiden13"), Val::from(1)])?;
                                    ctx.lines_as(
                                        "Wish Maiden",
                                        args![
                                            "You need to have 16 to 20 members present to open the gate of Okolnir.",
                                            "Come back when you are ready."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                }
                            }
                            2 => {
                                ctx.call(Function::Cutin, vec![Val::from("wish_maiden13"), Val::from(1)])?;
                                ctx.lines_as(
                                    "Wish Maiden",
                                    args![
                                        "Don't hesitate to try.",
                                        "You should catch the chance when it comes to you.",
                                        "Just gather your fellow members."
                                    ],
                                )?;
                                ctx.close_window()?;
                            }
                            _ => {}
                        }
                    }
                    2 => {
                        ctx.call(Function::Cutin, vec![Val::from("wish_maiden32"), Val::from(1)])?;
                        ctx.lines_as("Wish Maiden", args!["... Are you afraid of", "the trials facing you?", "...."])?;
                        ctx.next()?;
                        ctx.call(Function::Cutin, vec![Val::from("wish_maiden13"), Val::from(1)])?;
                        ctx.lines_as(
                            "Wish Maiden",
                            args![
                                "Do you think that you can defeat the Goddess shine easily?",
                                "I feel disappointed by all of you.",
                                "Just go away..."
                            ],
                        )?;
                        ctx.close_window()?;
                    }
                    _ => {}
                }
            } else {
                ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(1)])?;
                ctx.lines_as(
                    "Wish Maiden",
                    args![
                        "Do you wish to enter?",
                        "Only those prepared may enter here.",
                        "You must bring several items to enter Okolnir."
                    ],
                )?;
                ctx.next()?;
                ctx.mes("[Wish Maiden]")?;
                if runtime::compare(&l_sub_s.clone(), &Val::from("aru")).is_true() {
                    ctx.lines(args!["Dusk Glow", "Dawn Essence", "Cold Moonlight", "Hazy Starlight."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wish Maiden",
                        args![
                            "Please bring those four things, 10 Agate, 10 Rose Quartz, and 20 Elunium,",
                            "a Heavenly Maiden's Robe, as well as 100 Soft feathers."
                        ],
                    )?;
                } else {
                    ctx.lines(args![
                        "Goddess Tear",
                        "Valkyrie's Token",
                        "Brynhild Armor Piece",
                        "Hero Remains",
                        "Valkyrie Armor",
                        "Andvari's Ring."
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Wish Maiden",
                        args![
                            "You must bring those six things, 10 Gold, and 20 Elunium.",
                            "Many Valhala's Flowers are also required as an offering."
                        ],
                    )?;
                }
                ctx.next()?;
                ctx.lines_as("Wish Maiden", args!["Once all of those are prepared, the gate will open."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Wish Maiden",
                    args![
                        ".... ",
                        "...........The Goddess shines brightly down on you, you should be stronger to deserve it..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wish Maiden",
                    args![
                        "Remember...",
                        "You need to collect many soft feathers.",
                        "I hope that your dreams come true."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Wish Maiden",
                    args!["I will answer all your requests if you bring these to me."],
                )?;
                ctx.close_window()?;
            }
        } else {
            ctx.call(Function::Cutin, vec![Val::from("wish_maiden31"), Val::from(1)])?;
            ctx.lines_as(
                "Wish Maiden",
                args![
                    "I am... Wish maiden.",
                    "Mourning in this virtual realm, Okolnir.",
                    "On behalf of the humanbeings who defeated God here."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Wish Maiden",
                args!["Bring me the one who brought you to this place.", ".. Deliver him to my will."],
            )?;
            ctx.close_window()?;
        }
    } else {
        ctx.call(Function::Cutin, vec![Val::from("wish_maiden13"), Val::from(1)])?;
        ctx.lines_as("Wish Maiden", args!["...You are not qualified."])?;
        ctx.close_window()?;
        ctx.call(Function::PercentHeal, vec![Val::from(-100), Val::from(0)])?;
        ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(255)])?;
        return Err(Stop::End);
    }
    ctx.call(Function::Cutin, vec![Val::from("wish_maiden11"), Val::from(255)])?;
    return Err(Stop::End);
}

pub fn wish_maiden_gq_main(ctx: &Ctx) -> Script {
    wish_maiden_gq_main_body(ctx, Vec::new()).map(|_| ())
}

fn wish_maiden_gq_main_oninit_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if !(runtime::getd(
        ctx,
        &((Val::from("$siz_") + l_sub_s.clone()) + Val::from("_on")),
        &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )?
    .is_true())
    {
        ctx.call(Function::EnableNpc, vec![(Val::from("Wish Maiden#gq_") + l_sub_s.clone())])?;
    } else {
        ctx.call(Function::DisableNpc, vec![(Val::from("Wish Maiden#gq_") + l_sub_s.clone())])?;
    }
    return Err(Stop::End);
}

pub fn wish_maiden_gq_main_oninit(ctx: &Ctx) -> Script {
    wish_maiden_gq_main_oninit_body(ctx, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum Gate01GqMainStep {
    Start,
    OnTouch,
    SMonster,
    AfterSMonster,
    OnEnable,
    OnDisable,
    OnInit,
}

fn gate01_gq_main_run(ctx: &Ctx, mut step: Gate01GqMainStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_m = Val::from(0);
    let mut l_n: Vec<Val> = Vec::new();
    let mut l_point = Val::from(0);
    let mut l_saram = Val::from(0);
    let mut l_sub_s = Val::from("");
    'machine: loop {
        match step {
            Gate01GqMainStep::Start => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                step = Gate01GqMainStep::OnTouch;
                continue 'machine;
            }
            Gate01GqMainStep::OnTouch => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                l_saram = ctx.call(Function::GetMapUsers, vec![(Val::from("que_q") + l_sub_s.clone())])?;
                if l_saram.clone().number()? < 21 {
                    if runtime::getd(
                        ctx,
                        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_miro")),
                        &[
                            (".@m", runtime::Local::Scalar(&l_m)),
                            (".@n", runtime::Local::Array(&l_n)),
                            (".@point", runtime::Local::Scalar(&l_point)),
                            (".@saram", runtime::Local::Scalar(&l_saram)),
                            (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                        ],
                    )?
                    .loosely_equals(&runtime::getd(
                        ctx,
                        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_pcc")),
                        &[
                            (".@m", runtime::Local::Scalar(&l_m)),
                            (".@n", runtime::Local::Array(&l_n)),
                            (".@point", runtime::Local::Scalar(&l_point)),
                            (".@saram", runtime::Local::Scalar(&l_saram)),
                            (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                        ],
                    )?) {
                        l_point = ctx.call(Function::Rand, vec![Val::from(1), Val::from(5)])?;
                        let subject1 = l_point.clone();
                        if subject1 == 1 {
                            ctx.call(
                                Function::Warp,
                                vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(72), Val::from(271)],
                            )?;
                        } else if subject1 == 2 {
                            ctx.call(
                                Function::Warp,
                                vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(45), Val::from(243)],
                            )?;
                        } else if subject1 == 3 {
                            ctx.call(
                                Function::Warp,
                                vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(102), Val::from(248)],
                            )?;
                        } else if subject1 == 4 {
                            ctx.call(
                                Function::Warp,
                                vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(102), Val::from(300)],
                            )?;
                        } else if subject1 == 5 {
                            ctx.call(
                                Function::Warp,
                                vec![(Val::from("que_q") + l_sub_s.clone()), Val::from(46), Val::from(300)],
                            )?;
                        }
                        return Err(Stop::End);
                    }
                    let subject2 = runtime::getd(
                        ctx,
                        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_miro")),
                        &[
                            (".@m", runtime::Local::Scalar(&l_m)),
                            (".@n", runtime::Local::Array(&l_n)),
                            (".@point", runtime::Local::Scalar(&l_point)),
                            (".@saram", runtime::Local::Scalar(&l_saram)),
                            (".@sub$", runtime::Local::Scalar(&l_sub_s)),
                        ],
                    )?;
                    if subject2 == 0 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(77), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(271), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(72), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(271), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(1), false);
                    } else if subject2 == 1 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(63), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(278), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(63), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(282), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(2), false);
                    } else if subject2 == 2 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(63), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(294), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(59), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(294), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(3), false);
                    } else if subject2 == 3 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(50), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(300), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(46), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(300), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(4), false);
                    } else if subject2 == 4 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(51), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(280), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(51), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(285), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(5), false);
                    } else if subject2 == 5 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(51), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(258), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(51), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(262), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(6), false);
                    } else if subject2 == 6 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(49), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(243), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(45), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(243), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(7), false);
                    } else if subject2 == 7 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(86), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(249), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(82), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(249), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(8), false);
                    } else if subject2 == 8 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(102), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(243), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(102), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(248), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(9), false);
                    } else if subject2 == 9 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(90), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(256), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(90), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(260), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(10), false);
                    } else if subject2 == 10 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(90), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(283), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(90), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(280), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(11), false);
                    } else if subject2 == 11 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(102), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(295), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(102), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(300), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(12), false);
                    } else if subject2 == 12 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(96), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(285), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(96), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(290), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(13), false);
                    } else if subject2 == 13 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(63), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(278), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(63), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(282), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(14), false);
                    } else if subject2 == 14 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(65), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(243), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(61), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(243), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(15), false);
                    } else if subject2 == 15 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(73), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(249), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(70), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(249), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(16), false);
                    } else if subject2 == 16 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(102), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(275), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(102), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(282), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(17), false);
                    } else if subject2 == 17 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(70), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(300), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(66), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(300), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(18), false);
                    } else if subject2 == 18 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(57), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(255), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(57), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(258), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(19), false);
                    } else if subject2 == 19 {
                        let base = Val::from(0).number()?;
                        runtime::local_set(&mut l_n, &Val::from(base + 0), Val::from(84), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 1), Val::from(277), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 2), Val::from(84), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 3), Val::from(280), false);
                        runtime::local_set(&mut l_n, &Val::from(base + 4), Val::from(20), false);
                    }
                    gate01_gq_main_run(
                        ctx,
                        Gate01GqMainStep::SMonster,
                        vec![
                            l_sub_s.clone(),
                            runtime::local_get(&l_n, &Val::from(0), false),
                            runtime::local_get(&l_n, &Val::from(1), false),
                        ],
                    )?;
                    runtime::setd(
                        ctx,
                        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_miro")),
                        runtime::local_get(&l_n, &Val::from(4), false),
                        &mut [
                            (".@m", runtime::LocalMut::Scalar(&mut l_m)),
                            (".@n", runtime::LocalMut::Array(&mut l_n)),
                            (".@point", runtime::LocalMut::Scalar(&mut l_point)),
                            (".@saram", runtime::LocalMut::Scalar(&mut l_saram)),
                            (".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s)),
                        ],
                    )?;
                    ctx.call(
                        Function::Warp,
                        vec![
                            (Val::from("que_q") + l_sub_s.clone()),
                            runtime::local_get(&l_n, &Val::from(2), false),
                            runtime::local_get(&l_n, &Val::from(3), false),
                        ],
                    )?;
                    return Err(Stop::End);
                } else {
                    ctx.mes("There are too many people, you can't enter.")?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                step = Gate01GqMainStep::AfterSMonster;
                continue 'machine;
            }
            Gate01GqMainStep::SMonster => {
                let subject3 = ctx.var("BaseClass").get()?;
                if subject3 == 1 {
                    l_m = Val::from(1652);
                } else if subject3 == 2 {
                    l_m = Val::from(1663);
                } else if subject3 == 3 {
                    l_m = Val::from(1662);
                } else if subject3 == 4 {
                    l_m = Val::from(1661);
                } else if subject3 == 5 {
                    l_m = Val::from(1660);
                } else if subject3 == 6 {
                    l_m = Val::from(1659);
                } else {
                    l_m = Val::from(1652);
                }
                ctx.call(
                    Function::Monster,
                    vec![
                        (Val::from("que_q") + runtime::arg(&args, 0, Val::from(0))),
                        runtime::arg(&args, 1, Val::from(0)),
                        runtime::arg(&args, 2, Val::from(0)),
                        ctx.call(Function::StrCharInfo, vec![Val::from(0)])?,
                        l_m.clone(),
                        Val::from(1),
                        ((Val::from("#Gate_manager_") + runtime::arg(&args, 0, Val::from(0))) + Val::from("::OnMyMobDead")),
                    ],
                )?;
                return Ok(Val::from(0));
                return Ok(Val::from(0));
            }
            Gate01GqMainStep::AfterSMonster => {
                step = Gate01GqMainStep::OnEnable;
                continue 'machine;
            }
            Gate01GqMainStep::OnEnable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(Function::EnableNpc, vec![(Val::from("Gate01#gq_") + l_sub_s.clone())])?;
                ctx.call(
                    Function::SetCell,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from(58),
                        Val::from(302),
                        Val::from(63),
                        Val::from(302),
                        ctx.constant("CELL_WALKABLE")?,
                        Val::from(0),
                    ],
                )?;
                ctx.call(
                    Function::SetCell,
                    vec![
                        (Val::from("que_q") + l_sub_s.clone()),
                        Val::from(58),
                        Val::from(302),
                        Val::from(63),
                        Val::from(302),
                        ctx.constant("CELL_SHOOTABLE")?,
                        Val::from(0),
                    ],
                )?;
                return Err(Stop::End);
            }
            Gate01GqMainStep::OnDisable => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Gate01#gq_") + l_sub_s.clone())])?;
                ctx.call(
                    Function::KillMonster,
                    vec![(Val::from("que_q") + l_sub_s.clone()), Val::from("All")],
                )?;
                return Err(Stop::End);
            }
            Gate01GqMainStep::OnInit => {
                l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
                ctx.call(Function::DisableNpc, vec![(Val::from("Gate01#gq_") + l_sub_s.clone())])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn gate01_gq_main(ctx: &Ctx) -> Script {
    gate01_gq_main_run(ctx, Gate01GqMainStep::Start, Vec::new()).map(|_| ())
}

pub fn gate01_gq_main_ontouch(ctx: &Ctx) -> Script {
    gate01_gq_main_run(ctx, Gate01GqMainStep::OnTouch, Vec::new()).map(|_| ())
}

pub fn gate01_gq_main_onenable(ctx: &Ctx) -> Script {
    gate01_gq_main_run(ctx, Gate01GqMainStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn gate01_gq_main_ondisable(ctx: &Ctx) -> Script {
    gate01_gq_main_run(ctx, Gate01GqMainStep::OnDisable, Vec::new()).map(|_| ())
}

pub fn gate01_gq_main_oninit(ctx: &Ctx) -> Script {
    gate01_gq_main_run(ctx, Gate01GqMainStep::OnInit, Vec::new()).map(|_| ())
}

fn gate_manager_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn gate_manager_main(ctx: &Ctx) -> Script {
    gate_manager_main_body(ctx, Vec::new()).map(|_| ())
}

fn gate_manager_main_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if runtime::getd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_miro")),
        &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )?
    .loosely_equals(&runtime::getd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_pcc")),
        &[(".@sub$", runtime::Local::Scalar(&l_sub_s))],
    )?) && !(ctx
        .call(
            Function::MobCount,
            vec![
                (Val::from("que_q") + l_sub_s.clone()),
                ((Val::from("#Gate_manager_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
            ],
        )?
        .is_true())
    {
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#gq_miromob2_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
        )?;
    }
    return Err(Stop::End);
}

pub fn gate_manager_main_onmymobdead(ctx: &Ctx) -> Script {
    gate_manager_main_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn gq_miromob2_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn gq_miromob2_main(ctx: &Ctx) -> Script {
    gq_miromob2_main_body(ctx, Vec::new()).map(|_| ())
}

fn gq_miromob2_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gq_miromob2_main_onenable(ctx: &Ctx) -> Script {
    gq_miromob2_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn gq_miromob2_main_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden: How does it feel to see shadows of the past. This is only the beginning."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn gq_miromob2_main_ontimer1000(ctx: &Ctx) -> Script {
    gq_miromob2_main_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn gq_miromob2_main_ontimer6000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_c = Val::from(0);
    let mut l_i = Val::from(0);
    let mut l_mobname_s: Vec<Val> = Vec::new();
    let mut l_num: Vec<Val> = Vec::new();
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden: The things you seeing are not real, don't be caught in the Mystic garden."),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_mobname_s, &Val::from(base + 0), Val::from("Seyren Windsor"), true);
    runtime::local_set(&mut l_mobname_s, &Val::from(base + 1), Val::from("Kathryne Keyron"), true);
    runtime::local_set(&mut l_mobname_s, &Val::from(base + 2), Val::from("Cecil Damon"), true);
    runtime::local_set(&mut l_mobname_s, &Val::from(base + 3), Val::from("Margaretha Sorin"), true);
    runtime::local_set(&mut l_mobname_s, &Val::from(base + 4), Val::from("Eremes Guile"), true);
    runtime::local_set(&mut l_mobname_s, &Val::from(base + 5), Val::from("Howard Alt-Eisen"), true);
    let base = Val::from(0).number()?;
    runtime::local_set(&mut l_num, &Val::from(base + 0), Val::from(72), false);
    runtime::local_set(&mut l_num, &Val::from(base + 1), Val::from(271), false);
    runtime::local_set(&mut l_num, &Val::from(base + 2), Val::from(1640), false);
    runtime::local_set(&mut l_num, &Val::from(base + 3), Val::from(63), false);
    runtime::local_set(&mut l_num, &Val::from(base + 4), Val::from(282), false);
    runtime::local_set(&mut l_num, &Val::from(base + 5), Val::from(1645), false);
    runtime::local_set(&mut l_num, &Val::from(base + 6), Val::from(59), false);
    runtime::local_set(&mut l_num, &Val::from(base + 7), Val::from(294), false);
    runtime::local_set(&mut l_num, &Val::from(base + 8), Val::from(1644), false);
    runtime::local_set(&mut l_num, &Val::from(base + 9), Val::from(46), false);
    runtime::local_set(&mut l_num, &Val::from(base + 10), Val::from(300), false);
    runtime::local_set(&mut l_num, &Val::from(base + 11), Val::from(1643), false);
    runtime::local_set(&mut l_num, &Val::from(base + 12), Val::from(51), false);
    runtime::local_set(&mut l_num, &Val::from(base + 13), Val::from(285), false);
    runtime::local_set(&mut l_num, &Val::from(base + 14), Val::from(1641), false);
    runtime::local_set(&mut l_num, &Val::from(base + 15), Val::from(51), false);
    runtime::local_set(&mut l_num, &Val::from(base + 16), Val::from(262), false);
    runtime::local_set(&mut l_num, &Val::from(base + 17), Val::from(1642), false);
    l_c = Val::from(0);
    'l1: loop {
        if !(l_c.clone().number()? < 3) {
            break 'l1;
        }
        'b1: {
            l_i = Val::from(0);
            'l2: loop {
                if !(runtime::op(&l_i.clone(), "<", &Val::from(l_mobname_s.len() as i32))?.is_true()) {
                    break 'l2;
                }
                'b2: {
                    ctx.call(
                        Function::Monster,
                        vec![
                            (Val::from("que_q") + l_sub_s.clone()),
                            runtime::local_get(&l_num, &(l_i.clone().try_mul(Val::from(3))?), false),
                            runtime::local_get(&l_num, &((l_i.clone().try_mul(Val::from(3))?) + Val::from(1)), false),
                            runtime::local_get(&l_mobname_s, &l_i.clone(), true),
                            runtime::local_get(&l_num, &((l_i.clone().try_mul(Val::from(3))?) + Val::from(2)), false),
                            Val::from(1),
                            ((Val::from("#gq_miromob2_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
                        ],
                    )?;
                }
                l_i = (l_i.clone() + Val::from(1));
            }
            l_c = (l_c.clone() + Val::from(1));
        }
    }
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gq_miromob2_main_ontimer6000(ctx: &Ctx) -> Script {
    gq_miromob2_main_ontimer6000_body(ctx, Vec::new()).map(|_| ())
}

fn gq_miromob2_main_onreset_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::KillMonster,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#gq_miromob2_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
        ],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn gq_miromob2_main_onreset(ctx: &Ctx) -> Script {
    gq_miromob2_main_onreset_body(ctx, Vec::new()).map(|_| ())
}

fn gq_miromob2_main_onmymobdead_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    if ctx.call(
        Function::MobCount,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ((Val::from("#gq_miromob2_") + l_sub_s.clone()) + Val::from("::OnMyMobDead")),
        ],
    )? == 0
    {
        ctx.call(
            Function::MapAnnounce,
            vec![
                Val::from("que_qaru05"),
                Val::from("The Mystic garden exit is now open."),
                ctx.constant("BC_MAP")?,
                Val::from("0x00ff00"),
            ],
        )?;
        ctx.call(
            Function::SetCell,
            vec![
                (Val::from("que_q") + l_sub_s.clone()),
                Val::from(58),
                Val::from(302),
                Val::from(63),
                Val::from(302),
                ctx.constant("CELL_WALKABLE")?,
                Val::from(1),
            ],
        )?;
        ctx.call(
            Function::SetCell,
            vec![
                (Val::from("que_q") + l_sub_s.clone()),
                Val::from(58),
                Val::from(302),
                Val::from(63),
                Val::from(302),
                ctx.constant("CELL_SHOOTABLE")?,
                Val::from(1),
            ],
        )?;
        ctx.call(
            Function::DoNpcEvent,
            vec![((Val::from("#Maze_Manager_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
        )?;
    }
    return Err(Stop::End);
}

pub fn gq_miromob2_main_onmymobdead(ctx: &Ctx) -> Script {
    gq_miromob2_main_onmymobdead_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn okolnir_main(ctx: &Ctx) -> Script {
    okolnir_main_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Gate01#gq_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#Maze_Manager_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#event_start01_") + l_sub_s.clone()) + Val::from("::OnEnable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gd_") + l_sub_s.clone()) + Val::from("_mobctrl::OnEnable"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_01"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_02"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_03"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_04"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac01"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac02"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac03"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac04"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac01"))],
    )?;
    ctx.call(
        Function::EnableNpc,
        vec![((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac02"))],
    )?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn okolnir_main_onenable(ctx: &Ctx) -> Script {
    okolnir_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Wish Maiden#gq_") + l_sub_s.clone())])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gq_miromob2_") + l_sub_s.clone()) + Val::from("::OnReset"))],
    )?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Piamette#") + l_sub_s.clone())])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gdtimer01_") + l_sub_s.clone()) + Val::from("::OnStop"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnStop"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#piamette_") + l_sub_s.clone()) + Val::from("::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_gift::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gd_") + l_sub_s.clone()) + Val::from("_mobctrl::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Gate01#gq_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#Maze_Manager_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#event_start01_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nm_switch_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin02::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_02::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_03::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_04::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac02::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac03::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac04::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac02::OnDisable"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#to_agit_") + l_sub_s.clone()) + Val::from("_gate"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone01::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone02::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone03::OnReset"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage01"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage02"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage03"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage04"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage05"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage06"))],
    )?;
    ctx.call(Function::DisableNpc, vec![(Val::from("windpath03_") + l_sub_s.clone())])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("windpath04_") + l_sub_s.clone())])?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_miro")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_pcc")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn okolnir_main_ondisable(ctx: &Ctx) -> Script {
    okolnir_main_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_onstop_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn okolnir_main_onstop(ctx: &Ctx) -> Script {
    okolnir_main_onstop_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden : Do your best, Okolnir will disappear in one hour!"),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer1000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer1800000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 30 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer1800000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer1800000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer2400000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 20 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer2400000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer2400000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3000000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 10 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3000000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3000000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3300000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 5 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3300000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3300000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3360000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 4 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3360000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3360000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3420000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 3 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3420000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3420000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3480000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 2 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3480000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3480000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3540000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir will disappear in 1 minutes."),
            ctx.constant("BC_MAP")?,
            Val::from("0xff0000"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3540000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3540000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3600000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Okolnir has begun to disappear."),
            ctx.constant("BC_MAP")?,
            Val::from("0x4d4dff"),
        ],
    )?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Wish Maiden#gq_") + l_sub_s.clone())])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gq_miromob2_") + l_sub_s.clone()) + Val::from("::OnReset"))],
    )?;
    ctx.call(Function::DisableNpc, vec![(Val::from("Piamette#") + l_sub_s.clone())])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gdtimer01_") + l_sub_s.clone()) + Val::from("::OnStop"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gdtimer02_") + l_sub_s.clone()) + Val::from("::OnStop"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#piamette_") + l_sub_s.clone()) + Val::from("::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_boss::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Wish Maiden#") + l_sub_s.clone()) + Val::from("_gift::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#gd_") + l_sub_s.clone()) + Val::from("_mobctrl::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Gate01#gq_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#Maze_Manager_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#event_start01_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nm_switch_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin02::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#nmsom") + l_sub_s.clone()) + Val::from("_jin03::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_02::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_03::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Guard of Shadow#") + l_sub_s.clone()) + Val::from("_04::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac02::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac03::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Bloody Hunter#") + l_sub_s.clone()) + Val::from("_ac04::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac01::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("Temple Keeper#") + l_sub_s.clone()) + Val::from("_ac02::OnDisable"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#to_agit_") + l_sub_s.clone()) + Val::from("_gate"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone01::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone02::OnReset"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_stone03::OnReset"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage01"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage02"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage03"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage04"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage05"))],
    )?;
    ctx.call(
        Function::DisableNpc,
        vec![((Val::from("#") + l_sub_s.clone()) + Val::from("_cage06"))],
    )?;
    ctx.call(Function::DisableNpc, vec![(Val::from("windpath03_") + l_sub_s.clone())])?;
    ctx.call(Function::DisableNpc, vec![(Val::from("windpath04_") + l_sub_s.clone())])?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3600000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3600000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3605000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden: ... You will fall into a deep sleep within Okolnir... "),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3605000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3605000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3608000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::MapAnnounce,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            Val::from("Wish Maiden: ..Have courage ... and await your chance again... "),
            ctx.constant("BC_MAP")?,
            Val::from("0x00ff00"),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3608000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3608000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3610000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_miro")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_pcc")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_gd")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    runtime::setd(
        ctx,
        &((Val::from("$@gqse_") + l_sub_s.clone()) + Val::from("_nm")),
        Val::from(0),
        &mut [(".@sub$", runtime::LocalMut::Scalar(&mut l_sub_s))],
    )?;
    ctx.call(
        Function::MapWarp,
        vec![
            (Val::from("que_q") + l_sub_s.clone()),
            ctx.call(Function::StrNpcInfo, vec![Val::from(4)])?,
            Val::from(157),
            Val::from(369),
        ],
    )?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3610000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3610000_body(ctx, Vec::new()).map(|_| ())
}

fn okolnir_main_ontimer3611000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#okolnir_") + l_sub_s.clone()) + Val::from("_time01::OnEnable"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn okolnir_main_ontimer3611000(ctx: &Ctx) -> Script {
    okolnir_main_ontimer3611000_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    return Err(Stop::End);
}

pub fn maze_manager_main(ctx: &Ctx) -> Script {
    maze_manager_main_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_onenable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(Function::InitNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn maze_manager_main_onenable(ctx: &Ctx) -> Script {
    maze_manager_main_onenable_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ondisable_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_bf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_rf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_yf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    ctx.call(Function::StopNpcTimer, vec![])?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ondisable(ctx: &Ctx) -> Script {
    maze_manager_main_ondisable_body(ctx, Vec::new()).map(|_| ())
}

fn maze_manager_main_ontimer1000_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_sub_s = Val::from("");
    l_sub_s = shared::quests_okolnir::f_okolnir(ctx, vec![])?;
    ctx.call(
        Function::DoNpcEvent,
        vec![((Val::from("#miro_rf_") + l_sub_s.clone()) + Val::from("::OnDisable"))],
    )?;
    return Err(Stop::End);
}

pub fn maze_manager_main_ontimer1000(ctx: &Ctx) -> Script {
    maze_manager_main_ontimer1000_body(ctx, Vec::new()).map(|_| ())
}
