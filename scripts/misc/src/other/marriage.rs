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
enum WeddingStaffWStep {
    Start,
    End,
}

fn wedding_staff_w_run(ctx: &Ctx, mut step: WeddingStaffWStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_covenant = Val::from(0);
    let mut l_name_s = Val::from("");
    'machine: loop {
        match step {
            WeddingStaffWStep::Start => {
                ctx.fx().cutin("wedding_marry01", 2)?;
                if ctx.call(Function::CountItem, args![6026])? == 0 {
                    if (ctx.constant("VIP_SCRIPT")?.is_true()
                        && !(ctx.call(Function::VipStatus, args![constants::VIP_STATUS_ACTIVE])?.is_true()))
                    {
                        ctx.lines_as(
                            "Marry Happy",
                            args![
                                "Sorry but you can't get married right now.",
                                "The wedding is only available for subscribed players.",
                                "But if you have a ^FF0000Marriage Covenant^000000, then you'll be able to get married!"
                            ],
                        )?;
                        step = WeddingStaffWStep::End;
                        continue 'machine;
                    }
                } else {
                    l_covenant = Val::from(1);
                }
                if ctx.var("Upper").get()? == 2 {
                    ctx.lines_as(
                        "Marry Happy",
                        args![
                            "Hello~",
                            "My name is Marry Happy",
                            "and I'm here to provide you",
                            "with information related to",
                            "marriage. Now, did you",
                            "have any questions?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["I want to get married.", "I don't need your help!"])? == 0 {
                        ctx.lines_as(
                            "Marry Happy",
                            args![
                                "Oh, I'm sorry, but",
                                "adopted characters",
                                "aren't allowed to get",
                                "married. For now, why",
                                "don't you enjoy the simple",
                                "pleasures of childhood?"
                            ],
                        )?;
                        step = WeddingStaffWStep::End;
                        continue 'machine;
                    }
                    ctx.lines_as(
                        "Marry Happy",
                        args![
                            "Oh, of course you",
                            "don't! Little children",
                            "can't get married-- there",
                            "are too many laws against",
                            "that~ Aren't you the most",
                            "adorable little thing?"
                        ],
                    )?;
                    step = WeddingStaffWStep::End;
                    continue 'machine;
                }
                ctx.lines_as(
                    "Marry Happy",
                    args![
                        "Marriage is the beautiful",
                        "union of two souls that have",
                        "chosen to be together forever,",
                        "to share their joy and lives.",
                        "Is there a special someone",
                        "like that in your life?"
                    ],
                )?;
                ctx.next()?;
                'b1: {
                    match ctx.menu(&[
                        "Ask about Wedding Ceremony",
                        "Ask about Procedure",
                        "Apply for Wedding",
                        "We are the Invincible Single Army!",
                    ])? {
                        0 => {
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Wise and benevolent",
                                    "King Tristram III used to",
                                    "conduct wedding ceremonies,",
                                    "but he's no longer able to do",
                                    "so because of his royal duties",
                                    "and freneticly paced schedule."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Bishop Vomars, the bishop",
                                    "of love, is now the officiator",
                                    "of the marriage ceremony.",
                                    "He is truly a treasure to the",
                                    "Rune-Midgarts Kingdom."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "When you marry someone,",
                                    "it's for the rest of your life, so think carefully before making or",
                                    "accepting a marriage proposal.",
                                    "Keep in mind that a man can only",
                                    "marry a woman and vice versa."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "If you're lucky enough to",
                                    "find someone that you really",
                                    "want to spend the rest of your",
                                    "life with, you might want to pop the question. I hope everyone",
                                    "finds their perfect match~"
                                ],
                            )?;
                            break 'b1;
                        }
                        1 => {
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "The first part of the",
                                    "wedding procedure is to",
                                    "complete the application.",
                                    "Once the bride and bridegroom",
                                    "have finished applying, they",
                                    "must form a party of two."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "After forming a party of two,",
                                    "the couple must then speak to",
                                    "Bishop Vomars. The bridegroom",
                                    "speaks first and must tell his",
                                    "bride's exact name to the Bishop. Otherwise, the ceremony will stop."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Afterwards, the bride will",
                                    "speak to the Bishop and tell",
                                    "him the name of her groom.",
                                    "If these names are correctly",
                                    "told to the Bishop, they will",
                                    "be able to exchange rings."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Once the wedding rings are",
                                    "exchanged, the couple is forever bound in matrimony. Of course,",
                                    "before this point, there are many chances to change your mind, so..."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "If there are too many",
                                    "couples who want to get",
                                    "married at one time, please",
                                    "form a line and speak to Bishop",
                                    "Vomars in order since only one couple can be married at a time."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Finally, be sure to tell",
                                    "Bishop Vomars your partner's",
                                    "exact name without wasting too much time. If you take too long,",
                                    "the ceremony will automatically stop and you'll have to try again."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Brides need to remember",
                                    "that they only have 3 minutes",
                                    "to finish speaking to Bishop",
                                    "Vomars after their grooms",
                                    "have finished speaking to him."
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Thanks, that helps a lot!", "Easiest way to say my partner's name?"])? == 0 {
                                ctx.lines_as(
                                    "Marry Happy",
                                    args![
                                        "Well, I'm here to help",
                                        "weddings proceed as",
                                        "smoothly as possible.",
                                        "If there was something",
                                        "you didn't understand,",
                                        "feel free to ask me again."
                                    ],
                                )?;
                                step = WeddingStaffWStep::End;
                                continue 'machine;
                            }
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "The easiest way to write",
                                    "your partner's name for the",
                                    "bishop is to send a private",
                                    "message to your partner, and",
                                    "then left-click the name section that is left of the chat prompt."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Press the ''Ctrl'' and ''C''",
                                    "keys to copy the name. Then,",
                                    "you can paste the name into",
                                    "the input prompt by pressing",
                                    "the ''Insert'' and ''Shift'' keys. That sounds easy, right?"
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Alright, now let's try",
                                    "it. Practice giving me the",
                                    "name of your partner using",
                                    "the method I just described."
                                ],
                            )?;
                            ctx.next()?;
                            runtime::input_text(ctx, None, None)?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Alright, after you've",
                                    "decided to get married,",
                                    "come back to me and",
                                    "submit your application.",
                                    "I'll see you later, adventurer~"
                                ],
                            )?;
                            break 'b1;
                        }
                        2 => {
                            ctx.fx().cutin("wedding_marry02", 2)?;
                            if ctx.constant("VIP_SCRIPT")?.is_true() {
                                ctx.lines_as("Marry Happy", args!["^FF0000Wait!^000000", "^FF0000If your account in not a premium, you must have a Marriage Covenant to get married.^000000", "^FF0000Please check the current state of your account, and the one of your lover, before register for a wedding.^000000"])?;
                                ctx.next()?;
                            }
                            if ctx.var("Sex").get()? == constants::SEX_MALE {
                                ctx.lines_as(
                                    "Marry Happy",
                                    args!["So you'd like to get married?", "As a groom, you need to prepare"],
                                )?;
                                if l_covenant.clone().is_true() {
                                    ctx.lines(args![
                                        "^3377FF1 Tuxedo^000000 and give me your ^3377FF1 Marriage Covenant^000000.",
                                        "Brides have to provide their own Wedding Dresses and pay a fee",
                                        "of 1,200,000 zeny or also bring a Marriage Covenant."
                                    ])?;
                                } else {
                                    ctx.lines(args![
                                        "^3377FF1 Tuxedo^000000 and pay ^3377FF1,300,000 zeny^000000.",
                                        "Brides have to provide their own Wedding Dresses and pay a fee",
                                        "of 1,200,000 zeny."
                                    ])?;
                                }
                            } else {
                                ctx.lines_as(
                                    "Marry Happy",
                                    args!["So you'd like to get married?", "As a bride, you need to prepare"],
                                )?;
                                if l_covenant.clone().is_true() {
                                    ctx.lines(args![
                                        "^3377FF1 Wedding Dress^000000 and give me your ^3377FFMarriage Covenant^000000.",
                                        "Grooms must bring a Tuxedo and pay",
                                        "1,300,000 zeny or also bring a Marriage Covenant."
                                    ])?;
                                } else {
                                    ctx.lines(args![
                                        "^3377FF1 Wedding Dress^000000 and pay a fee of 1,200,000 zeny. Grooms must",
                                        "bring a Tuxedo and pay 1,300,000 zeny to get married."
                                    ])?;
                                }
                            }
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "Brides and grooms also need",
                                    "to have ^3377FF1 Diamond Ring^000000 to be",
                                    "exchanged with their partners.",
                                    "You'll need all of these items",
                                    "prepared when you submit your",
                                    "wedding ceremony application."
                                ],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "The prospective bride and",
                                    "groom must both complete",
                                    "application process before",
                                    "the wedding can take place.",
                                    "Now, would you like to",
                                    "apply for marriage?"
                                ],
                            )?;
                            ctx.next()?;
                            if ctx.menu(&["Yes", "No"])? == 0 {
                                if ctx.call(Function::GetPartnerId, vec![])?.is_true() {
                                    ctx.fx().cutin("wedding_marry02", 2)?;
                                    ctx.lines_as(
                                        "Marry Happy",
                                        args![
                                            "I'm sorry, but you can't",
                                            "apply for another marriage!",
                                            "I can't allow you to betray",
                                            "your spouse like that, and",
                                            "besides, polygamy isn't",
                                            "legal here in Rune-Midgarts."
                                        ],
                                    )?;
                                    break 'b1;
                                } else if ctx.var("wedding_sign").get()? == 1 {
                                    ctx.lines_as(
                                        "Marry Happy",
                                        args![
                                            "Didn't you already",
                                            "complete the application?",
                                            "Hmm, make sure that your",
                                            "partner also finished the",
                                            "application process, and",
                                            "then talk to Bishop Vomars."
                                        ],
                                    )?;
                                    break 'b1;
                                } else if ctx.player().base_level()? < 45 {
                                    ctx.lines_as(
                                        "Marry Happy",
                                        args![
                                            "Hmm, you need to be",
                                            "strong enough to protect",
                                            "the one that you love before",
                                            "you can consider marriage.",
                                            "After you grow stronger,",
                                            "come and talk to me again."
                                        ],
                                    )?;
                                    step = WeddingStaffWStep::End;
                                    continue 'machine;
                                } else if ctx.items().count(2613)? < 1 {
                                    ctx.lines_as(
                                        "Marry Happy",
                                        args![
                                            "Mm? Did you forget to",
                                            "bring the Diamond Ring",
                                            "to exchange with your partner",
                                            "during the wedding ceremony?",
                                            "Look for it carefully and come",
                                            "back after you find it, okay?"
                                        ],
                                    )?;
                                    step = WeddingStaffWStep::End;
                                    continue 'machine;
                                } else if ctx.var("Sex").get()? == constants::SEX_FEMALE {
                                    if (ctx.call(Function::CountItem, args![6026])? == 0 && ctx.player().zeny()? < 1200000) {
                                        ctx.lines_as(
                                            "Marry Happy",
                                            args![
                                                "I'm sorry, but all brides",
                                                "must pay the 1,200,000",
                                                "zeny fee or bring a Marriage",
                                                "Covenant to proceed with the",
                                                "wedding ceremony. Perhaps",
                                                "you could ask your partner",
                                                "to help you with the funds?"
                                            ],
                                        )?;
                                        step = WeddingStaffWStep::End;
                                        continue 'machine;
                                    } else if ctx.items().count(2338)? < 1 {
                                        ctx.lines_as(
                                            "Marry Happy",
                                            args![
                                                "Oh dear, did you forget",
                                                "your Wedding Dress?",
                                                "Hurry and find it, then",
                                                "bring it to me-- you",
                                                "absolutely need it",
                                                "for the wedding!"
                                            ],
                                        )?;
                                        step = WeddingStaffWStep::End;
                                        continue 'machine;
                                    }
                                } else if ctx.var("Sex").get()? == constants::SEX_MALE {
                                    if (ctx.call(Function::CountItem, args![6026])? == 0 && ctx.player().zeny()? < 1300000) {
                                        ctx.lines_as(
                                            "Marry Happy",
                                            args![
                                                "I'm sorry, but you don't",
                                                "have the 1,300,000 zeny",
                                                "or the Marriage Covenant",
                                                "that all grooms must pay",
                                                "for the wedding ceremony.",
                                                "Did you misplace your money?"
                                            ],
                                        )?;
                                        step = WeddingStaffWStep::End;
                                        continue 'machine;
                                    } else if ctx.items().count(7170)? < 1 {
                                        ctx.lines_as(
                                            "Marry Happy",
                                            args![
                                                "Where's your Tuxedo?",
                                                "You absolutely have to",
                                                "wear it during the wedding",
                                                "ceremony! Find it, bring it",
                                                "to me, and then we can finally",
                                                "begin the wedding, okay?"
                                            ],
                                        )?;
                                        step = WeddingStaffWStep::End;
                                        continue 'machine;
                                    }
                                }
                                ctx.lines_as(
                                    "Marry Happy",
                                    args![
                                        "Well, it looks like you",
                                        "have everything ready.",
                                        "Although I'm not sure who",
                                        "your partner is, let me be",
                                        "the first to congratulate you",
                                        "on your upcoming wedding~"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Marry Happy",
                                    args!["Now, let's begin the", "application. Please write", "down your exact name here."],
                                )?;
                                ctx.next()?;
                                loop {
                                    let (input, _) = runtime::input_text(ctx, None, None)?;
                                    l_name_s = input;
                                    if l_name_s.loosely_equals(&ctx.call(Function::StrCharInfo, args![0])?) {
                                        break;
                                    }
                                    ctx.lines_as(
                                        "Marry Happy",
                                        args![
                                            "Hmmm, you have to write",
                                            "down your name exactly as",
                                            "it is displayed. Maybe you",
                                            "need to copy and paste it?",
                                            "Anyway, let's try it again."
                                        ],
                                    )?;
                                    ctx.next()?;
                                }
                                ctx.lines_as(
                                    "Marry Happy",
                                    args![
                                        "Great, it looks like we",
                                        "finished your application.",
                                        "Remember that you'll need",
                                        "to tell Bishop Vomars your",
                                        "partner's exact name when",
                                        "you talk to him later, okay?"
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Marry Happy",
                                    args![
                                        "When your partner is",
                                        "finished with the application",
                                        "process, both of you should",
                                        "speak to the Bishop to begin",
                                        "the wedding ceremony."
                                    ],
                                )?;
                                ctx.npc().emotion(constants::ET_THROB)?;
                                ctx.next()?;
                                ctx.mes("[Marry Happy]")?;
                                if ctx.var("Sex").get()? == constants::SEX_FEMALE {
                                    ctx.lines(args![
                                        "Since you're the bride,",
                                        "you need to wait for the",
                                        "groom to speak to Bishop",
                                        "Vomars first. When he's",
                                        "finished, it'll be your turn",
                                        "to speak to Bishop Vomars."
                                    ])?;
                                    if l_covenant.clone().is_true() {
                                        ctx.items().take(6026, 1)?;
                                    } else {
                                        ctx.player().set_zeny(ctx.player().zeny()? - 1200000)?;
                                    }
                                    ctx.items().take(2338, 1)?;
                                } else {
                                    ctx.lines(args![
                                        "Since you're the groom,",
                                        "you need to speak to the",
                                        "Bishop first. When you're",
                                        "finished, it will be your",
                                        "bride's turn to speak to",
                                        "Bishop Vomars."
                                    ])?;
                                    if l_covenant.clone().is_true() {
                                        ctx.items().take(6026, 1)?;
                                    } else {
                                        ctx.player().set_zeny(ctx.player().zeny()? - 1300000)?;
                                    }
                                    ctx.items().take(7170, 1)?;
                                }
                                ctx.items().take(2613, 1)?;
                                ctx.var("wedding_sign").set(Val::from(1))?;
                                step = WeddingStaffWStep::End;
                                continue 'machine;
                            }
                            ctx.lines_as(
                                "Marry Happy",
                                args![
                                    "No...?",
                                    "Well, when you're",
                                    "ready for marriage,",
                                    "feel free to come back to",
                                    "me so that you can apply,",
                                    "okay? Have a good day~"
                                ],
                            )?;
                            break 'b1;
                        }
                        3 => {
                            ctx.fx().cutin("wedding_marry02", 2)?;
                            ctx.npc().do_event("Single Army#Prontera::OnEnable")?;
                            ctx.npc().do_event("Single Army#Geffen::OnEnable")?;
                            ctx.npc().do_event("Single Army#Morocc::OnEnable")?;
                            ctx.npc().do_event("Single Army#Payon::OnEnable")?;
                            ctx.npc().do_event("Single Army#Amatsu::OnEnable")?;
                            ctx.npc().do_event("Single Army#Kunlun::OnEnable")?;
                            ctx.npc().emotion(constants::ET_HUK)?;
                            ctx.lines_as(
                                "Single Army",
                                args!["^CC9933You have to refine", "items on your own to", "make great equipment!^000000"],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                args![constants::ET_ROCK, ctx.call(Function::GetNpcId, args![0, "Single Army#Prontera"])?],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Single Army",
                                args![
                                    "^330099It's a waste to",
                                    "form parties in",
                                    "dungeons! I can",
                                    "make it on my own!^000000"
                                ],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                args![constants::ET_ROCK, ctx.call(Function::GetNpcId, args![0, "Single Army#Geffen"])?],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Single Army",
                                args![
                                    "^666666Hell, I've trained",
                                    "all by myself since",
                                    "birth, all the way",
                                    "to my job change!^000000"
                                ],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                args![constants::ET_ROCK, ctx.call(Function::GetNpcId, args![0, "Single Army#Morocc"])?],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Single Army",
                                args![
                                    "^666600I CHOOSE to spend",
                                    "Christmas alone...",
                                    "playing Solitaire and",
                                    "doing crossword puzzles!^000000"
                                ],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                args![constants::ET_ROCK, ctx.call(Function::GetNpcId, args![0, "Single Army#Payon"])?],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Single Army",
                                args![
                                    "^CC9966Women may break my",
                                    "spirit, but they'll never",
                                    "take... my FREEDOM!^000000"
                                ],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                args![constants::ET_ROCK, ctx.call(Function::GetNpcId, args![0, "Single Army#Amatsu"])?],
                            )?;
                            ctx.next()?;
                            ctx.lines_as(
                                "Single Army",
                                args!["^669900...We're the free! We're", "the Invincible Single Army!^000000"],
                            )?;
                            ctx.call(
                                Function::Emotion,
                                args![constants::ET_ROCK, ctx.call(Function::GetNpcId, args![0, "Single Army#Kunlun"])?],
                            )?;
                            ctx.close_window()?;
                            ctx.fx().cutin("wedding_marry01", 255)?;
                            ctx.npc().emotion(constants::ET_SWEAT)?;
                            ctx.npc().do_event("Single Army#Prontera::OnInit")?;
                            ctx.npc().do_event("Single Army#Geffen::OnInit")?;
                            ctx.npc().do_event("Single Army#Morocc::OnInit")?;
                            ctx.npc().do_event("Single Army#Payon::OnInit")?;
                            ctx.npc().do_event("Single Army#Amatsu::OnInit")?;
                            ctx.npc().do_event("Single Army#Kunlun::OnInit")?;
                            return Err(Stop::End);
                        }
                        _ => {}
                    }
                }
                ctx.close_window()?;
                ctx.fx().cutin("wedding_marry01", 255)?;
                return Err(Stop::End);
            }
            WeddingStaffWStep::End => {
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn wedding_staff_w(ctx: &Ctx) -> Script {
    wedding_staff_w_run(ctx, WeddingStaffWStep::Start, Vec::new()).map(|_| ())
}

pub fn single_army_prontera(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Single Army",
        args!["^CC9933You have to refine", "items on your own to", "make great equipment!^000000"],
    )?;
    return ctx.close();
}

pub fn single_army_prontera_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Prontera", false)?;
    return ctx.end();
}

pub fn single_army_prontera_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Prontera", true)?;
    ctx.npc().emotion(constants::ET_GO)?;
    return ctx.end();
}

pub fn single_army_geffen(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Single Army",
        args![
            "^330099It's a waste to",
            "form parties in",
            "dungeons! I can",
            "make it on my own!^000000"
        ],
    )?;
    return ctx.close();
}

pub fn single_army_geffen_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Geffen", false)?;
    return ctx.end();
}

pub fn single_army_geffen_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Geffen", true)?;
    ctx.npc().emotion(constants::ET_GO)?;
    return ctx.end();
}

pub fn single_army_morocc(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Single Army",
        args![
            "^666666Hell, I've trained",
            "all by myself since",
            "birth, all the way",
            "to my job change!^000000"
        ],
    )?;
    return ctx.close();
}

pub fn single_army_morocc_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Morocc", false)?;
    return ctx.end();
}

pub fn single_army_morocc_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Morocc", true)?;
    ctx.npc().emotion(constants::ET_GO)?;
    return ctx.end();
}

pub fn single_army_payon(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Single Army",
        args![
            "^666600I CHOOSE to spend",
            "Christmas alone...",
            "playing Solitaire and",
            "doing crossword puzzles!^000000"
        ],
    )?;
    return ctx.close();
}

pub fn single_army_payon_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Payon", false)?;
    return ctx.end();
}

pub fn single_army_payon_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Payon", true)?;
    ctx.npc().emotion(constants::ET_GO)?;
    return ctx.end();
}

pub fn single_army_amatsu(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Single Army",
        args![
            "^CC9966Women may break my",
            " spirit, but they'll never",
            "take... my FREEDOM!^000000"
        ],
    )?;
    return ctx.close();
}

pub fn single_army_amatsu_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Amatsu", false)?;
    return ctx.end();
}

pub fn single_army_amatsu_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Amatsu", true)?;
    ctx.npc().emotion(constants::ET_GO)?;
    return ctx.end();
}

pub fn single_army_kunlun(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Single Army",
        args!["^669900...We're the free! We're", "the Invincible Single Army!^000000"],
    )?;
    return ctx.close();
}

pub fn single_army_kunlun_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Kunlun", false)?;
    return ctx.end();
}

pub fn single_army_kunlun_onenable(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Single Army#Kunlun", true)?;
    ctx.npc().emotion(constants::ET_GO)?;
    return ctx.end();
}

#[derive(Clone, Copy, Debug)]
enum BishopWStep {
    Start,
    End,
    SBusy,
    OnStop,
    OnReset,
    OnTimer180000,
}

fn bishop_w_run(ctx: &Ctx, mut step: BishopWStep, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_partymembercount = Val::from(0);
    'machine: loop {
        match step {
            BishopWStep::Start => {
                ctx.fx().cutin("wedding_bomars01", 2)?;
                if ctx.var("Upper").get()? == 2 {
                    ctx.lines_as(
                        "Vomars",
                        args![
                            "Greetings, child.",
                            "Are you lost? Hmmm.",
                            "Do you know where your",
                            "mommy and daddy are?"
                        ],
                    )?;
                    ctx.close_window()?;
                    ctx.fx().cutin("wedding_bomars01", 255)?;
                    return Err(Stop::End);
                }
                if !(ctx.call(Function::GetPartnerId, vec![])?.is_true()) {
                    if !(ctx.var("$@wedding").get()?.is_true()) {
                        if ctx.var("wedding_sign").get()? == 1 {
                            runtime::party_members(ctx, ctx.call(Function::GetCharacterId, args![1])?, Val::from(0))?;
                            l_partymembercount = ctx.var("$@partymembercount").get()?;
                            if l_partymembercount == 2 {
                                if ctx.var("Sex").get()? == constants::SEX_MALE {
                                    ctx.var("$@wedding").set(Val::from(1))?;
                                    ctx.call(Function::InitNpcTimer, vec![])?;
                                    ctx.lines_as(
                                        "Vomars",
                                        args![
                                            "Young lovers, please",
                                            "remember this moment for",
                                            "the rest of your lives. May your future be blessed with peace",
                                            "and joy. May the love you share",
                                            "grow with each passing day."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.call(
                                        Function::MapAnnounce,
                                        args![
                                            "prt_church",
                                            ((Val::from("It's the marriage proposal from the groom, Mr. ")
                                                + ctx.call(Function::StrCharInfo, args![0])?)
                                                + Val::from("...")),
                                            constants::BC_MAP
                                        ],
                                    )?;
                                    ctx.lines_as(
                                        "Vomars",
                                        args![
                                            "Until the end of the",
                                            "world, may you stand",
                                            "by the side of the one",
                                            "whom you love, to support",
                                            "her and protect her. Now, may",
                                            "I know the name of your bride?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                    let (input, _) = runtime::input_text(ctx, None, None)?;
                                    ctx.var("$@wed_bride$").set(input)?;
                                    ctx.lines_as(
                                        "Vomars",
                                        args![
                                            ((Val::from("Mr. ") + ctx.call(Function::StrCharInfo, args![0])?) + Val::from("...")),
                                            "Do you swear on your life",
                                            "that you will forever cherish",
                                            "and care for your bride,",
                                            ((Val::from("Miss ") + ctx.var("$@wed_bride$").get()?) + Val::from("?"))
                                        ],
                                    )?;
                                    ctx.next()?;
                                    let choice = runtime::select_values(ctx, &[Val::from("I do.")])?;
                                    ctx.var("@menu").set(choice)?;
                                    ctx.var("$@wed_groom$").set(ctx.call(Function::StrCharInfo, args![0])?)?;
                                    ctx.lines_as(
                                        "Vomars",
                                        args![
                                            "Now, it is time for",
                                            "your bride to make",
                                            "her wedding vows.",
                                            "If she will come forward..."
                                        ],
                                    )?;
                                    ctx.close_window()?;
                                    ctx.call(
                                        Function::MapAnnounce,
                                        args![
                                            "prt_church",
                                            ((((Val::from("The groom, Mr. ") + ctx.call(Function::StrCharInfo, args![0])?)
                                                + Val::from(", has made his vows to Miss "))
                                                + ctx.var("$@wed_bride$").get()?)
                                                + Val::from("...")),
                                            constants::BC_MAP
                                        ],
                                    )?;
                                    ctx.fx().cutin("", 255)?;
                                    return Err(Stop::End);
                                }
                                ctx.lines_as(
                                    "Vomars",
                                    args![
                                        "I'm sorry, but the groom",
                                        "must speak to me first in",
                                        "order to begin the wedding.",
                                        "It's old fashioned protocol,",
                                        "but I'll admit that it does",
                                        "keep things running smoothly."
                                    ],
                                )?;
                                step = BishopWStep::End;
                                continue 'machine;
                            }
                            ctx.lines_as(
                                "Vomars",
                                args![
                                    "Before you can be",
                                    "married, you must",
                                    "first form a party of",
                                    "two with your partner.",
                                    "Then, we can proceed",
                                    "with the ceremony."
                                ],
                            )?;
                            step = BishopWStep::End;
                            continue 'machine;
                        }
                        ctx.lines_as(
                            "Vomars",
                            args![
                                "You must apply for",
                                "marriage with Happy Marry",
                                "before you can get married.",
                                "Happy Marry will let you know",
                                "what else you'll need to do",
                                "to prepare for marriage."
                            ],
                        )?;
                        step = BishopWStep::End;
                        continue 'machine;
                    } else if ctx.var("$@wedding").get()? == 1 {
                        if ctx.var("wedding_sign").get()? == 1 {
                            runtime::party_members(ctx, ctx.call(Function::GetCharacterId, args![1])?, Val::from(0))?;
                            l_partymembercount = ctx.var("$@partymembercount").get()?;
                            if l_partymembercount == 2 {
                                if ctx.var("Sex").get()? == constants::SEX_FEMALE {
                                    if ctx
                                        .call(Function::StrCharInfo, args![0])?
                                        .loosely_equals(&ctx.var("$@wed_bride$").get()?)
                                    {
                                        ctx.lines_as(
                                            "Vomars",
                                            args![
                                                "Young lovers, please",
                                                "remember this moment for",
                                                "the rest of your lives. May your future be blessed with peace",
                                                "and joy. May the love you share",
                                                "grow with each passing day."
                                            ],
                                        )?;
                                        ctx.next()?;
                                        ctx.call(
                                            Function::MapAnnounce,
                                            args![
                                                "prt_church",
                                                ((Val::from("Let's hear what the bride, Miss ") + ctx.var("$@wed_bride$").get()?)
                                                    + Val::from(", has to say...")),
                                                constants::BC_MAP
                                            ],
                                        )?;
                                        ctx.lines_as(
                                            "Vomars",
                                            args![
                                                ((Val::from("Miss ") + ctx.var("$@wed_bride$").get()?) + Val::from("...")),
                                                "Do you swear to stay",
                                                ((Val::from("true to ") + ctx.var("$@wed_groom$").get()?) + Val::from(",")),
                                                "to be by his side, no matter",
                                                "what the dangers may be?"
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if ctx.menu(&["^FF0000No.^000000", "I do."])? == 0 {
                                            ctx.fx().cutin("wedding_bomars03", 2)?;
                                            ctx.call(
                                                Function::MapAnnounce,
                                                args!["prt_church", "Next couple, please proceed...", constants::BC_MAP],
                                            )?;
                                            ctx.lines_as(
                                                "Vomars",
                                                args![
                                                    (Val::from("So ") + ctx.var("$@wed_groom$").get()?),
                                                    "isn't the one you",
                                                    "want to marry? Hmm.",
                                                    "I'm truly sorry for this",
                                                    "misunderstanding..."
                                                ],
                                            )?;
                                            ctx.var("$@wedding").set(Val::from(0))?;
                                            ctx.close_window()?;
                                            ctx.call(Function::StopNpcTimer, vec![])?;
                                            ctx.fx().cutin("", 255)?;
                                            return Err(Stop::End);
                                        }
                                        ctx.lines_as(
                                            "Vomars",
                                            args![
                                                "Do you truly swear",
                                                "fidelity and patience?",
                                                ((Val::from("Will you marry ") + ctx.var("$@wed_groom$").get()?) + Val::from("?"))
                                            ],
                                        )?;
                                        ctx.next()?;
                                        if ctx.menu(&["Yes, I do.", "^FF0000No.^000000"])? == 0 {
                                            if ctx
                                                .call(
                                                    Function::IsLoggedIn,
                                                    args![ctx.call(Function::GetCharacterId, args![3, ctx.var("$@wed_groom$").get()?],)?],
                                                )?
                                                .is_true()
                                                && ctx.call(Function::Marriage, args![ctx.var("$@wed_groom$").get()?])?.is_true()
                                            {
                                                ctx.call(Function::Wedding, vec![])?;
                                                ctx.call(Function::StartStatus, args![ctx.constant("SC_WEDDING")?, 3600000, 1])?;
                                                ctx.items().give(2635, 1)?;
                                                ctx.call(
                                                    Function::AttachRid,
                                                    args![ctx.call(Function::GetCharacterId, args![3, ctx.var("$@wed_groom$").get()?],)?],
                                                )?;
                                                ctx.call(Function::StartStatus, args![ctx.constant("SC_WEDDING")?, 3600000, 1])?;
                                                ctx.items().give(2634, 1)?;
                                                ctx.call(Function::DetachRid, vec![])?;
                                                ctx.call(
                                                    Function::AttachRid,
                                                    args![ctx.call(Function::GetCharacterId, args![3, ctx.var("$@wed_bride$").get()?],)?],
                                                )?;
                                                ctx.fx().cutin("wedding_bomars02", 2)?;
                                                ctx.call(
                                                    Function::MapAnnounce,
                                                    args![
                                                        "prt_church",
                                                        ((((Val::from("I now pronounce you, ") + ctx.var("$@wed_groom$").get()?)
                                                            + Val::from(" and "))
                                                            + ctx.var("$@wed_bride$").get()?)
                                                            + Val::from(", husband and wife.")),
                                                        constants::BC_MAP
                                                    ],
                                                )?;
                                                ctx.lines_as(
                                                    "Vomars",
                                                    args![
                                                        "By the power invested",
                                                        "in me as Royal Bishop of",
                                                        "the Rune-Midgarts Kingdom,",
                                                        "I now pronounce you husband",
                                                        "and wife. May your future be",
                                                        "blessed with many great joys."
                                                    ],
                                                )?;
                                                ctx.next()?;
                                                ctx.lines_as(
                                                    "Vomars",
                                                    args![
                                                        "And lastly...",
                                                        "Always be happy,",
                                                        ((Val::from("dear ") + ctx.var("$@wed_bride$").get()?) + Val::from("..."))
                                                    ],
                                                )?;
                                                ctx.var("$@wed_groom$").set(Val::from(""))?;
                                                ctx.var("$@wed_bride$").set(Val::from(""))?;
                                                ctx.var("$@wedding").set(Val::from(0))?;
                                                ctx.close_window()?;
                                                ctx.call(Function::StopNpcTimer, vec![])?;
                                                ctx.fx().cutin("", 255)?;
                                                ctx.call(Function::DetachRid, vec![])?;
                                                return Err(Stop::End);
                                            }
                                            ctx.fx().cutin("wedding_bomars03", 2)?;
                                            ctx.lines_as(
                                                "Vomars",
                                                args![
                                                    "Hm. It seems that",
                                                    "your groom left before",
                                                    "the ceremony has finished",
                                                    "Please try again once he's",
                                                    "returned."
                                                ],
                                            )?;
                                            step = BishopWStep::End;
                                            continue 'machine;
                                        } else {
                                            ctx.fx().cutin("wedding_bomars03", 2)?;
                                            ctx.call(
                                                Function::MapAnnounce,
                                                args![
                                                    "prt_church",
                                                    ((((Val::from("Alas! ") + ctx.var("$@wed_bride$").get()?)
                                                        + Val::from(" has rejected "))
                                                        + ctx.var("$@wed_groom$").get()?)
                                                        + Val::from("'s marriage proposal!")),
                                                    constants::BC_MAP
                                                ],
                                            )?;
                                            ctx.lines_as(
                                                "Vomars",
                                                args![
                                                    "Hm. It seems that",
                                                    "you've changed your",
                                                    "mind. Although I feel",
                                                    "sorry for the groom, you",
                                                    "must do what your heart",
                                                    "tells you is right. Now, run!"
                                                ],
                                            )?;
                                        }
                                        ctx.var("$@wed_groom$").set(Val::from(""))?;
                                        ctx.var("$@wed_bride$").set(Val::from(""))?;
                                        ctx.var("$@wedding").set(Val::from(0))?;
                                        ctx.close_window()?;
                                        ctx.call(Function::StopNpcTimer, vec![])?;
                                        ctx.fx().cutin("", 255)?;
                                        return Err(Stop::End);
                                    }
                                    bishop_w_run(ctx, BishopWStep::SBusy, vec![])?;
                                }
                                bishop_w_run(ctx, BishopWStep::SBusy, vec![])?;
                            }
                            bishop_w_run(ctx, BishopWStep::SBusy, vec![])?;
                        }
                        if ctx
                            .call(Function::StrCharInfo, args![0])?
                            .loosely_equals(&ctx.var("$@wed_bride$").get()?)
                        {
                            ctx.lines_as(
                                "Vomars",
                                args![
                                    "Hm? It appears that",
                                    "Happy Marry still hasn't",
                                    "received your marriage",
                                    "application. Please speak",
                                    "to her so that we can begin",
                                    "the wedding ceremony."
                                ],
                            )?;
                            step = BishopWStep::End;
                            continue 'machine;
                        }
                        bishop_w_run(ctx, BishopWStep::SBusy, vec![])?;
                    }
                    bishop_w_run(ctx, BishopWStep::SBusy, vec![])?;
                }
                ctx.lines_as(
                    "Vomars",
                    args![
                        "I wish you eternal",
                        "happiness. No matter",
                        "how dark the present may",
                        "be, always stand by your",
                        "loved one's side and look",
                        "to the future with hope."
                    ],
                )?;
                step = BishopWStep::End;
                continue 'machine;
            }
            BishopWStep::End => {
                ctx.close_window()?;
                ctx.fx().cutin("", 255)?;
                return Err(Stop::End);
            }
            BishopWStep::SBusy => {
                if (ctx.var("$@wed_groom$").get()? != "" && ctx.var("$@wed_bride$").get()? != "") {
                    ctx.lines_as(
                        "Vomars",
                        args![
                            "The wedding of",
                            ((Val::from("Miss ") + ctx.var("$@wed_bride$").get()?) + Val::from(" and")),
                            (Val::from("Mister ") + ctx.var("$@wed_groom$").get()?),
                            "is currently in progress.",
                            "Please keep your voice down."
                        ],
                    )?;
                    step = BishopWStep::End;
                    continue 'machine;
                }
                ctx.lines_as(
                    "Vomars",
                    args![
                        "I'm conducting a wedding",
                        "for another couple now, so",
                        "please wait patiently for your",
                        "turn. Thanks for understanding..."
                    ],
                )?;
                step = BishopWStep::End;
                continue 'machine;
            }
            BishopWStep::OnStop => {
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
            BishopWStep::OnReset => {
                ctx.var("$@wed_groom$").set(Val::from(""))?;
                ctx.var("$@wed_bride$").set(Val::from(""))?;
                ctx.var("$@wedding").set(Val::from(0))?;
                return Err(Stop::End);
            }
            BishopWStep::OnTimer180000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prt_church",
                        "You've responded too slowly... Next couple, please proceed.",
                        constants::BC_MAP
                    ],
                )?;
                ctx.npc().do_event("Bishop#w::OnReset")?;
                ctx.call(Function::StopNpcTimer, vec![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn bishop_w(ctx: &Ctx) -> Script {
    bishop_w_run(ctx, BishopWStep::Start, Vec::new()).map(|_| ())
}

pub fn bishop_w_onstop(ctx: &Ctx) -> Script {
    bishop_w_run(ctx, BishopWStep::OnStop, Vec::new()).map(|_| ())
}

pub fn bishop_w_onreset(ctx: &Ctx) -> Script {
    bishop_w_run(ctx, BishopWStep::OnReset, Vec::new()).map(|_| ())
}

pub fn bishop_w_ontimer180000(ctx: &Ctx) -> Script {
    bishop_w_run(ctx, BishopWStep::OnTimer180000, Vec::new()).map(|_| ())
}

fn the_king_of_rune_midgarts_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as(
        "Vomars",
        args!["Wh-who are you?", "You must know the", "password to invoke", "my awesome powers."],
    )?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, args![1854, 0])?.number()? < 1 {
        ctx.lines_as("Vomars", args!["This is", "no place for", "fooling around."])?;
        ctx.close_window()?;
        ctx.warp("prt_church", 101, 102)?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Wedding Switch",
        args![
            "Is there a problem with",
            "the wedding ceremony?",
            "I can reset the Bishop",
            "Vomars NPC if you like."
        ],
    )?;
    ctx.next()?;
    'b1: {
        match ctx.menu(&["No, thanks", "RESET"])? {
            0 => {
                ctx.lines_as(
                    "Wedding Switch",
                    args![
                        "Alright, then.",
                        "However, if the",
                        "Bishop Vomars",
                        "NPC is stuck, it may",
                        "be best to reset it."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            1 => {
                ctx.npc().do_event("Bishop#w::OnStop")?;
                ctx.npc().do_event("Bishop#w::OnReset")?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "prt_church",
                        "You've responded too slowly... Next couple, please proceed.",
                        constants::BC_MAP
                    ],
                )?;
                ctx.lines_as(
                    "Wedding Switch",
                    args![
                        "The Bishop Vomars NPC",
                        "has now been reactivated.",
                        "It should now be possible",
                        "to proceed with weddings."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn the_king_of_rune_midgarts(ctx: &Ctx) -> Script {
    the_king_of_rune_midgarts_body(ctx, Vec::new()).map(|_| ())
}

fn divorce_staff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Bad Ending", args!["Uh oh...", "You know I can't", "do anything for you."])?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, args![1854, 0])?.number()? < 1 {
        ctx.lines_as("Bad Ending", args!["Hmm...", "You really", "shouldn't be", "in this place..."])?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Bad Ending",
        args![
            "Great, you know the",
            "password! Now, did you",
            "want me to remove the",
            "Wedding Ring in your",
            "inventory?"
        ],
    )?;
    ctx.next()?;
    'b1: {
        match ctx.menu(&["Drop 1 Wedding Ring.", "Keep it."])? {
            0 => {
                let l_ring = if ctx.var("Sex").get()? == constants::SEX_MALE {
                    Val::from(2634)
                } else {
                    Val::from(2635)
                };
                if ctx.call(Function::CountItem, args![l_ring.clone()])?.is_true() {
                    ctx.call(Function::DelItem, args![l_ring.clone(), 1])?;
                    ctx.lines_as("Bad Ending", args!["It's done!"])?;
                } else {
                    ctx.lines_as(
                        "Bad Ending",
                        args![
                            "I couldn't find",
                            "the Wedding Ring...",
                            "Please make sure",
                            "that it's not equipped."
                        ],
                    )?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            1 => {
                ctx.lines_as(
                    "Bad Ending",
                    args![
                        "You sure you want",
                        "to keep that ring?",
                        "Alright, but if it becomes",
                        "a problem, you come to me."
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn divorce_staff(ctx: &Ctx) -> Script {
    divorce_staff_body(ctx, Vec::new()).map(|_| ())
}

fn remarry_staff_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    shared::other_gm_npcs::f_gm_npc(ctx, vec![])?;
    ctx.lines_as("Wedding Again", args!["Hmm...?", "What exactly are", "you doing here?"])?;
    ctx.next()?;
    if shared::other_gm_npcs::f_gm_npc(ctx, args![1854, 0])?.number()? < 1 {
        ctx.lines_as(
            "Wedding Again",
            args!["Ahk!", "An adventurer", "like you shouldn't", "be in this place!"],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    if ctx.call(Function::GetPartnerId, vec![])?.is_true() {
        ctx.lines_as(
            "Wedding Again",
            args!["Hmm...", "I can only create", "a Wedding Ring if the", "character is married."],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    }
    ctx.lines_as(
        "Wedding Again",
        args![
            "Ah, I see that you",
            "know the password.",
            "Alright, if you somehow",
            "lost your Wedding Ring,",
            "I can make you a new one."
        ],
    )?;
    ctx.next()?;
    'b1: {
        match ctx.menu(&["Make new Wedding Ring.", "Cancel."])? {
            0 => {
                let l_ring = if ctx.var("Sex").get()? == constants::SEX_MALE {
                    Val::from(2634)
                } else {
                    Val::from(2635)
                };
                if (ctx.call(Function::CountItem, args![l_ring.clone()])?.is_true()
                    || ctx.call(Function::IsEquipped, args![l_ring.clone()])?.is_true())
                {
                    ctx.lines_as(
                        "Wedding Again",
                        args![
                            "Wait, wait...",
                            "You're wearing your",
                            "Wedding Ring. I better",
                            "not make you another since",
                            "you don't need more than one."
                        ],
                    )?;
                } else {
                    ctx.call(Function::GetItem, args![l_ring.clone(), 1])?;
                    ctx.lines_as("Wedding Again", args!["Here you go~", "It's your brand", "new Wedding Ring!"])?;
                }
                ctx.close_window()?;
                return Err(Stop::End);
            }
            1 => {
                ctx.lines_as(
                    "Wedding Again",
                    args![
                        "Alright. If you ever",
                        "lose your Wedding Ring,",
                        "come to me if you happen",
                        "to need a new one, okay?"
                    ],
                )?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
            _ => {}
        }
    }
    Ok(Val::from(0))
}

pub fn remarry_staff(ctx: &Ctx) -> Script {
    remarry_staff_body(ctx, Vec::new()).map(|_| ())
}
