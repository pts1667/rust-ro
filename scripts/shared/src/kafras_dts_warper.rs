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

pub fn f_votekafra(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    crate::other_global_functions::f_cleargarbage(ctx, vec![])?;
    let choice = runtime::arg(&args, 0, Val::from(0));
    if choice == 1 {
        ctx.fx().cutin("kafra_01", 2)?;
    } else if choice == 2 {
        ctx.fx().cutin("kafra_02", 2)?;
    } else if choice == 3 {
        ctx.fx().cutin("kafra_03", 2)?;
    } else if choice == 4 {
        ctx.fx().cutin("kafra_04", 2)?;
    } else if choice == 5 {
        ctx.fx().cutin("kafra_05", 2)?;
    } else if choice == 6 {
        ctx.fx().cutin("kafra_06", 2)?;
    } else if choice == 7 {
        ctx.fx().cutin("kafra_07", 2)?;
    } else if choice == 8 {
        ctx.fx().cutin("kafra_08", 2)?;
    } else if choice == 9 {
        ctx.fx().cutin("kafra_09", 2)?;
    }
    ctx.lines_as(
        "Kafra Voting Staff",
        args![
            "Greetings, adventurer.",
            "As you may be aware, we",
            "are holding an election to",
            "determine which company will",
            "provide the Dungeon Teleport",
            "Service. How may I help you?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Reason for Election", "Cast a Vote", "Use Teleport Service", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Kafra Voting Staff",
                args![
                    "Cool Event Corp and the",
                    "Kafra Corporation have both",
                    "been planning to provide a",
                    "Teleport Service to dungeons."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Voting Staff",
                args![
                    "But due to technological",
                    "limitations, only one company",
                    "can serve as provider for this",
                    "Dungeon Teleport Service at a",
                    "time. There, both companies have agreed to hold special elections."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Voting Staff",
                args![
                    "Each company has its own",
                    "policies and guarantees in",
                    "regards to the Dungeon Teleport Service, and in this election, the",
                    "customers will ultimately decide and choose what's best for them."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Voting Staff",
                args![
                    "For now, the Dungeon",
                    "Teleport Service will be",
                    "provided in a series of trial periods. This way, customers can",
                    "see the benefits of both companies before making the final decision."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Voting Staff",
                args![
                    "If you are qualified,",
                    "please vote in each election",
                    "to decide which company will",
                    "provide the Dungeon Teleport",
                    "Service for the next trial period. Thank you for your support~"
                ],
            )?;
        }
        1 => {
            if ctx.var("$dts_result").get()?.number()? == 0 {
                if ctx.var("lhz_vote").get()?.number()? <= ctx.var("$dts_votecount").get()?.number()? {
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "We, the Kafra Corporation,",
                            "are planning to provide the",
                            "Dungeon Teleport Service",
                            "to the following dungeons..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![" ", " "])?;
                    if ctx.var("$dts_kafrawins").get()?.number()? == 0 {
                        ctx.mes("^FF0000Toy Factory, Level 2^000000")?;
                    } else if ctx.var("$dts_kafrawins").get()?.number()? == 1 {
                        ctx.lines(args![
                            "^FF0000Toy Factory, Level 2^000000",
                            "^FF0000Al De Baran Clock Tower, Level 3 ^000000"
                        ])?;
                    } else {
                        ctx.lines(args![
                            "^FF0000Toy Factory, Level 2^000000",
                            "^FF0000Al De Baran Clock Tower, Level 3 Lava Dungeon, Level 2^000000"
                        ])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "If you are interested in",
                            "a Teleport Service to this",
                            "area, then please vote for",
                            "us. Would you like to vote",
                            "for the Kafra Corporation?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["No", "Yes"])? == 0 {
                        ctx.lines_as(
                            "Kafra Voting Staff",
                            args![
                                "I understand. But if you",
                                "happen to change your mind,",
                                "you are welcome to come back",
                                "at any time. Thank you and",
                                "have a good day, adventurer."
                            ],
                        )?;
                    } else {
                        ctx.var("lhz_vote").set(Val::from(ctx.var("$dts_votecount").get()?.number()? + 1))?;
                        ctx.var("$dts_kafravotes")
                            .set(Val::from(ctx.var("$dts_kafravotes").get()?.number()? + 1))?;
                        ctx.lines_as(
                            "Kafra Voting Staff",
                            args![
                                "Thanks for your vote!",
                                "We'll continue to do our best",
                                "to provide the highest quality",
                                "service to our customers. Have",
                                "a good day and remember that the Kafra service is on your side~"
                            ],
                        )?;
                    }
                } else {
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "I'm sorry, but you've ",
                            "already participated in",
                            "this election. When the next",
                            "election comes, you will be",
                            "able to vote once again.",
                            "Thank you for your support~"
                        ],
                    )?;
                }
            } else if ctx.var("$dts_result").get()?.number()? == 3 {
                if ctx.var("lhz_vote").get()?.number()? <= ctx.var("$dts_votecount").get()?.number()? {
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "After totalling the number of",
                            "votes from the last election,",
                            "we have concluded that the",
                            "minimum voter participation",
                            "condition was not satisfied."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "Therefore, another election to",
                            "determine which company will",
                            "provide the Dungeon Teleport",
                            "Service will be held. The Kafra",
                            "Corporation will teleport to",
                            "the following dungeons..."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines(args![" ", " "])?;
                    if ctx.var("$dts_kafrawins").get()?.number()? == 0 {
                        ctx.mes("^FF0000Toy Factory, Level 2^000000")?;
                    } else if ctx.var("$dts_kafrawins").get()?.number()? == 1 {
                        ctx.lines(args![
                            "^FF0000Toy Factory, Level 2^000000",
                            "^FF0000Al De Baran Clock Tower, Level 3 ^000000"
                        ])?;
                    } else {
                        ctx.lines(args![
                            "^FF0000Toy Factory, Level 2^000000",
                            "^FF0000Al De Baran Clock Tower, Level 3 Lava Dungeon, Level 2^000000"
                        ])?;
                    }
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "If you are interested in",
                            "a Teleport Service to these",
                            "areas, then please vote for",
                            "us. Would you like to vote",
                            "for the Kafra Corporation?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["No", "Yes"])? == 0 {
                        ctx.lines_as(
                            "Kafra Voting Staff",
                            args![
                                "I understand. But if you",
                                "happen to change your mind,",
                                "you are welcome to come back",
                                "at any time. Thank you and",
                                "have a good day, adventurer."
                            ],
                        )?;
                    } else {
                        ctx.var("lhz_vote").set(Val::from(ctx.var("$dts_votecount").get()?.number()? + 1))?;
                        ctx.var("$dts_kafravotes")
                            .set(Val::from(ctx.var("$dts_kafravotes").get()?.number()? + 1))?;
                        ctx.lines_as(
                            "Kafra Voting Staff",
                            args![
                                "Thanks for your vote!",
                                "We'll continue to do our best",
                                "to provide the highest quality",
                                "service to our customers. Have",
                                "a good day and remember that the Kafra service is on your side~"
                            ],
                        )?;
                    }
                } else {
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "After totalling the number of",
                            "votes from the last election,",
                            "we have concluded that the",
                            "minimum voter participation",
                            "condition was not satisfied."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "Therefore, another election to",
                            "determine which company will",
                            "provide the Dungeon Teleport",
                            "Service will be held. However,",
                            "since you've already voted, you cannot vote again in this election."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Kafra Voting Staff",
                        args![
                            "Your participation in these",
                            "elections is much appreciated,",
                            "and we encourage you to vote",
                            "again during the next election.",
                            "Thank you and have a nice day~"
                        ],
                    )?;
                }
            } else {
                ctx.lines_as(
                    "Kafra Voting Staff",
                    args![
                        "I'm sorry, but there are",
                        "no elections taking place at",
                        "this time. When the polls are",
                        "open, we encourage you to take",
                        "part and voice your opinions.",
                        "Thank you for your support~"
                    ],
                )?;
            }
        }
        2 => {
            if ctx.var("$dts_result").get()?.number()? == 2 {
                ctx.lines_as(
                    "Kafra Voting Staff",
                    args![
                        "Thank you for choosing the",
                        "Dungeon Teleport Service.",
                        "Please keep in mind that the",
                        "Free Warp Tickets and Kafra",
                        "Special Reserve Points do not",
                        "apply in this special service."
                    ],
                )?;
                ctx.next()?;
                if ctx.var("$dts_kafrawins").get()?.number()? == 0 {
                    if ctx.menu(&["Toy Factory, Level 2 -> 4,000 z", "Cancel"])? == 0 {
                        if ctx.player().zeny()? >= 4000 {
                            ctx.fx().cutin("", 255)?;
                            ctx.player().set_zeny(ctx.player().zeny()? - 4000)?;
                            ctx.warp("xmas_dun02", 130, 123)?;
                            return Err(Stop::End);
                        } else {
                            ctx.lines_as(
                                "Kafra Voting Staff",
                                args![
                                    "I'm sorry, but you don't",
                                    "have enough money to pay",
                                    "the 4,000 zeny fee to teleport",
                                    "to the Toy Factory. Please",
                                    "check your funds again."
                                ],
                            )?;
                        }
                    } else {
                        ctx.lines_as("Kafra Voting Staff", args!["We, here at Kafra Corporation,", "are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence."])?;
                    }
                } else if ctx.var("$dts_kafrawins").get()?.number()? == 1 {
                    match ctx.menu(&["Toy Factory, Level 2 -> 4,000 z", "Clock Tower, Level 3 -> 4,000 z", "Cancel"])? {
                        0 => {
                            if ctx.player().zeny()? >= 4000 {
                                ctx.player().set_zeny(ctx.player().zeny()? - 4000)?;
                                ctx.fx().cutin("kafra_09", 255)?;
                                ctx.warp("xmas_dun02", 130, 123)?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Kafra Voting Staff",
                                    args![
                                        "I'm sorry, but you don't",
                                        "have enough money to pay",
                                        "the 4,000 zeny fee to teleport",
                                        "to the Toy Factory. Please",
                                        "check your funds again."
                                    ],
                                )?;
                            }
                        }
                        1 => {
                            if ctx.player().zeny()? >= 4000 {
                                ctx.player().set_zeny(ctx.player().zeny()? - 4000)?;
                                ctx.fx().cutin("kafra_09", 255)?;
                                ctx.warp("alde_dun03", 265, 22)?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Kafra Voting Staff",
                                    args![
                                        "I'm sorry, but you don't",
                                        "have enough money to pay",
                                        "the 4,000 zeny fee to teleport",
                                        "to the Clock Tower. Please",
                                        "check your funds again."
                                    ],
                                )?;
                            }
                        }
                        _ => {
                            ctx.lines_as("Kafra Voting Staff", args!["We, here at Kafra Corporation,", "are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence."])?;
                        }
                    }
                } else {
                    match ctx.menu(&[
                        "Toy Factory, Level 2 -> 4,000 z",
                        "Clock Tower, Level 3 -> 4,000 z",
                        "Lava Dungeon, Level 2 -> 4,000 z",
                        "Cancel",
                    ])? {
                        0 => {
                            if ctx.player().zeny()? >= 4000 {
                                ctx.player().set_zeny(ctx.player().zeny()? - 4000)?;
                                ctx.fx().cutin("kafra_09", 255)?;
                                ctx.warp("xmas_dun02", 130, 123)?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Kafra Voting Staff",
                                    args![
                                        "I'm sorry, but you don't",
                                        "have enough money to pay",
                                        "the 4,000 zeny fee to teleport",
                                        "to the Toy Factory. Please",
                                        "check your funds again."
                                    ],
                                )?;
                            }
                        }
                        1 => {
                            if ctx.player().zeny()? >= 4000 {
                                ctx.player().set_zeny(ctx.player().zeny()? - 4000)?;
                                ctx.fx().cutin("kafra_09", 255)?;
                                ctx.warp("alde_dun03", 265, 22)?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Kafra Voting Staff",
                                    args![
                                        "I'm sorry, but you don't",
                                        "have enough money to pay",
                                        "the 4,000 zeny fee to teleport",
                                        "to the Clock Tower. Please",
                                        "check your funds again."
                                    ],
                                )?;
                            }
                        }
                        2 => {
                            if ctx.player().zeny()? >= 4000 {
                                ctx.player().set_zeny(ctx.player().zeny()? - 4000)?;
                                ctx.fx().cutin("kafra_09", 255)?;
                                ctx.warp("mag_dun02", 47, 40)?;
                                return Err(Stop::End);
                            } else {
                                ctx.lines_as(
                                    "Kafra Voting Staff",
                                    args![
                                        "I'm sorry, but you don't",
                                        "have enough money to pay",
                                        "the 4,000 zeny fee to teleport",
                                        "to the Lava Dungeon. Please",
                                        "check your funds again."
                                    ],
                                )?;
                            }
                        }
                        _ => {
                            ctx.lines_as("Kafra Voting Staff", args!["We, here at Kafra Corporation,", "are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence."])?;
                        }
                    }
                }
            } else if ctx.var("$dts_result").get()?.number()? == 1 {
                ctx.lines_as(
                    "Kafra Voting Staff",
                    args![
                        "I'm sorry, but because of",
                        "the results from the most",
                        "recent election, Cool Event",
                        "Corp. is currently handling",
                        "the Dungeon Teleport Service. We apologize for the inconvenience."
                    ],
                )?;
            } else {
                ctx.lines_as(
                    "Kafra Voting Staff",
                    args![
                        "I'm sorry, but the",
                        "Dungeon Teleport Service",
                        "is not active during the voting",
                        "period. Once the election is",
                        "over, the Dungeon Teleport",
                        "Service will become available."
                    ],
                )?;
            }
        }
        3 => {
            ctx.lines_as("Kafra Voting Staff", args!["We, here at Kafra Corporation,", "are always endeavoring to provide you with the best services. We hope that we meet your adventuring needs and standards of excellence."])?;
        }
        _ => {}
    }
    ctx.close_window()?;
    ctx.fx().cutin("", 255)?;
    Err(Stop::End)
}
