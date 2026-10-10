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

pub fn vendigos(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Vendigos",
        args![
            "Welcome to the world of Arena, the battle against a time limit.",
            "My name is Vendigos, I am here to help you."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Vendigos", args!["If you have any questions, feel free to ask me."])?;
    ctx.next()?;
    loop {
        match ctx.menu(&["How to challenge", "About Arena Points", "My Current Arena Points", "Cancel"])? {
            0 => {
                ctx.lines_as(
                    "Vendigos",
                    args!["There are two different kinds of arena mode such as ^3131FFPlayer Mode^000000 and ^3131FFParty Mode^000000."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vendigos",
                    args![
                        "^3131FFPlayer Mode^000000 consists of 4 different stages based on character level from 50~80.",
                        "For a party with 5 members, they can participate in ^3131FFParty Mode^000000."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Vendigos", args!["For ^3131FFeach Player Mode Stage^000000, characters who are 20 levels higher than the level requirement on each stage cannot enter. This is to prevent high level characters preoccupying a low level stage."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Vendigos",
                    args![
                        "There are the NPCs for Player Mode Stages from 12 o'clock direction to the reversed clock direction.",
                        "While you're waiting in a chat room, you will be automatically guided to an arena room by the waiting order."
                    ],
                )?;
            }
            1 => {
                ctx.lines_as("Vendigos", args!["Let me explain about the ^3131FFArena Point^000000.", "It is a reward point for players who ^3131FFsuccessfully cleared a arena stage^000000, a player can possess the maximum 30,000 points."])?;
                ctx.next()?;
                ctx.lines_as("Vendigos", args!["Even if you did not clear a stage due to time-over or other errors, you will be given a very small amount of arena points."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Vendigos",
                    args![
                        "Regarding the use of Arena Points,",
                        "we are providing various services such as souvenir photograph services through an NPC named Givu."
                    ],
                )?;
            }
            2 => {
                ctx.lines_as(
                    "Vendigos",
                    args![Val::from("Let me check ") + ctx.player().name()? + "'s current arena points."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vendigos",
                    args![Val::from("") + ctx.player().name()? + " has total ^3131FF" + ctx.var("arena_point").get()? + "^000000 points."],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Vendigos",
                    args!["If you wish to know how to use arena points, please refer to the 2nd menu ^3131FF'About Arena Points'^000000."],
                )?;
            }
            _ => {
                ctx.lines_as("Vendigos", args!["Okay then,", "please have", "a good time."])?;
                return ctx.close();
            }
        }
        ctx.next()?;
        ctx.lines_as("Vendigos", args!["Is there anything", "more I can help you with?"])?;
        ctx.next()?;
    }
}

fn announce_arena_record(ctx: &Ctx, record: Val) -> Script {
    ctx.call(Function::MapAnnounce, args!["arena_room", record, 0])?;
    ctx.end()
}

pub fn vendigos_onlinerec_50(ctx: &Ctx) -> Script {
    announce_arena_record(
        ctx,
        Val::from("")
            + ctx.var("$arena_50topn$").get()?
            + " has renewed the top record in the Arena Time Force Battle level 50. Congratulations!",
    )
}

pub fn vendigos_onlinerec_60(ctx: &Ctx) -> Script {
    announce_arena_record(
        ctx,
        Val::from("")
            + ctx.var("$arena_60topn$").get()?
            + " has renewed the top record in the Arena Time Force Battle level 60. Congratulations!",
    )
}

pub fn vendigos_onlinerec_70(ctx: &Ctx) -> Script {
    announce_arena_record(
        ctx,
        Val::from("")
            + ctx.var("$arena_70topn$").get()?
            + " has renewed the top record in the Arena Time Force Battle level 70. Congratulations!",
    )
}

pub fn vendigos_onlinerec_80(ctx: &Ctx) -> Script {
    announce_arena_record(
        ctx,
        Val::from("")
            + ctx.var("$arena_80topn$").get()?
            + " has renewed the top record in the Arena Time Force Battle level 80. Congratulations!",
    )
}

pub fn vendigos_onlinerec_pt(ctx: &Ctx) -> Script {
    announce_arena_record(
        ctx,
        Val::from("Party ")
            + ctx.var("$arena_pttopn$").get()?
            + " has renewed the top record in the Arena Time Force Battle. Congratulations!",
    )
}

pub fn arena_record_staff(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Owen Kheuv",
        args!["Hello, my name is Own Kheuv", "in charge of every arena stage record of players."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Owen Kheuv",
        args![
            "Would you like to check the top record players in each stage?",
            "If so, please choose a menu below."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Lv 50 Stage", "Lv 60 Stage", "Lv 70 Stage", "Lv 80 Stage", "Party Stage"])? {
        0 => {
            ctx.lines_as(
                "Owen Kheuv",
                args![
                    Val::from("Currently the top player of the arena Lv 50 stage is ^3131FF ")
                        + ctx.var("$arena_50topn$").get()?
                        + "^000000, the top record is ^3131FF"
                        + ctx.var("$top_50min").get()?
                        + "^000000 minutes ^3131FF"
                        + ctx.var("$top_50sec").get()?
                        + "^000000 seconds."
                ],
            )?;
            ctx.close()
        }
        1 => {
            ctx.lines_as(
                "Owen Kheuv",
                args![
                    Val::from("Currenly the top player of the arena Lv 60 stage is ^3131FF")
                        + ctx.var("$arena_60topn$").get()?
                        + "^000000, the top record is ^3131FF"
                        + ctx.var("$top_60min").get()?
                        + "^000000 minutes ^3131FF"
                        + ctx.var("$top_60sec").get()?
                        + "^000000 seconds."
                ],
            )?;
            ctx.close()
        }
        2 => {
            ctx.lines_as(
                "Owen Kheuv",
                args![
                    Val::from("Currenly the top player of the arena Lv 70 stage is ^3131FF")
                        + ctx.var("$arena_70topn$").get()?
                        + "^000000, the top record is ^3131FF"
                        + ctx.var("$top_70min").get()?
                        + "^000000 minutes ^3131FF"
                        + ctx.var("$top_70sec").get()?
                        + "^000000 seconds."
                ],
            )?;
            ctx.close()
        }
        3 => {
            ctx.lines_as(
                "Owen Kheuv",
                args![
                    Val::from("Currenly the top player of the arena Lv 80 stage is ^3131FF")
                        + ctx.var("$arena_80topn$").get()?
                        + "^000000, the top record is ^3131FF"
                        + ctx.var("$top_80min").get()?
                        + "^000000 minutes ^3131FF"
                        + ctx.var("$top_80sec").get()?
                        + "^000000 seconds."
                ],
            )?;
            ctx.close()
        }
        _ => {
            ctx.lines_as(
                "Owen Kheuv",
                args![
                    Val::from("Currenly the top party is ^3131FF")
                        + ctx.var("$arena_pttopn$").get()?
                        + "^000000, the top record is ^3131FF"
                        + ctx.var("$top_ptmin").get()?
                        + "^000000 minutes ^3131FF"
                        + ctx.var("$top_ptsec").get()?
                        + "^000000 seconds."
                ],
            )?;
            ctx.close()
        }
    }
}

pub fn helper_pat(ctx: &Ctx) -> Script {
    ctx.lines_as("Pat", args!["Welcome, welcome.", "I am a helper of the Lv 50 arena stage."])?;
    ctx.next()?;
    ctx.lines_as(
        "Pat",
        args![
            "This ^3131FFLv 50 arena stage^000000",
            "is accessable to characters from ^FF0000level 50^000000 to ^FF0000level 69^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pat",
        args![
            "This level limitation is to prevent high level characters abusing low level arena stages. I hope you will understand.",
            "Also we accept an entrance fee, 1,000 zeny."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Pat", args!["Let me introduce you about the play rules of arena."])?;
    ctx.next()?;
    ctx.lines_as(
        "Pat",
        args![
            "^3131FFWait in a chat room for your turn coming.^000000",
            "When it's your turn, you will be automatically warped to an arena map."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Pat",
        args![
            "As immediately as you enter, a timer to check your play time will be activated.",
            "Please follow what ^3131FFHeel and Toe^000000 guides you. "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Pat", args!["You have a ^3131FF5 minutes^000000 battle time."])?;
    ctx.next()?;
    ctx.lines_as("Pat", args!["After you clear every room including a boss room, you will be warped again to an ^3131FFending waiting room^000000.", "You can only allow to stay in the waiting room for ^3131FF1 minute^000000, please hurry up to receive the arena points and leave."])?;
    ctx.next()?;
    ctx.lines_as("Pat", args!["If you stay over 1 minute inside the ending waiting room, you will be forced outside and will not receive any arena points. Please remember that."])?;
    ctx.next()?;
    ctx.lines_as("Pat", args!["I hope you will have a good time."])?;
    ctx.close()
}

pub fn helper_ben(ctx: &Ctx) -> Script {
    ctx.lines_as("Ben", args!["Welcome, welcome.", "I am a helper of the Lv 60 arena stage."])?;
    ctx.next()?;
    ctx.lines_as(
        "Ben",
        args![
            "This ^3131FFLv 60 arena stage^000000",
            "is accessable to characters from ^FF0000level 60^000000 to ^FF0000level 79^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ben",
        args![
            "This level limitation is to prevent high level characters abusing low level arena stages. I hope you will understand.",
            "Also we accept an entrance fee, 1,000 zeny."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Ben", args!["Let me introduce you about the play rules of arena."])?;
    ctx.next()?;
    ctx.lines_as(
        "Ben",
        args![
            "^3131FFWait in a chat room for your turn coming.^000000",
            "When it's your turn, you will be automatically warped to an arena map."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Ben",
        args![
            "As immediately as you enter, a timer to check your play time will be activated.",
            "Please follow what ^3131FFMinilover^000000 guides you. "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Ben", args!["You have a ^3131FF6 minutes^000000 battle time."])?;
    ctx.next()?;
    ctx.lines_as("Ben", args!["After you clear every room including a boss room, you will be warped again to an ^3131FFending waiting room^000000.", "You can only allow to stay in the waiting room for ^3131FF1 minute^000000, please hurry up to receive the arena points and leave."])?;
    ctx.next()?;
    ctx.lines_as("Ben", args!["If you stay over 1 minute inside the ending waiting room, you will be forced outside and will not receive any arena points. Please remember that."])?;
    ctx.next()?;
    ctx.lines_as("Ben", args!["I hope you will have a good time."])?;
    ctx.close()
}

pub fn helper_vicious(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Vicious",
        args![
            "Hey there.",
            "My name is Vicious, I am a helper of Lv 70 arena stage.",
            "(...I have no clue how the hell I put myself into this crappy work...mumble mumble...grumble grumble..)"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Vicious",
        args![
            "This ^3131FFLv 70 arena stage^000000",
            "is accessable to characters from ^FF0000level 70^000000 to ^FF0000level 89^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Vicious",
        args![
            "This level limitation is to prevent high level characters abusing low level arena stages. I hope you will understand.",
            "Also we accept an entrance fee, 1,000 zeny."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Vicious", args!["Let me introduce you about the play rules of arena."])?;
    ctx.next()?;
    ctx.lines_as(
        "Vicious",
        args![
            "^3131FFWait in a chat room for your turn coming.^000000",
            "When it's your turn, you will be automatically warped to an arena map."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Vicious",
        args![
            "As immediately as you enter, a timer to check your play time will be activated.",
            "Please follow what ^3131FFCadilac^000000 guides you. "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Vicious", args!["You have a ^3131FF7 minutes^000000 battle time."])?;
    ctx.next()?;
    ctx.lines_as("Vicious", args!["After you clear every room including a boss room, you will be warped again to an ^3131FFending waiting room^000000.", "You can only allow to stay in the waiting room for ^3131FF1 minute^000000, please hurry up to receive the arena points and leave."])?;
    ctx.next()?;
    ctx.lines_as("Vicious", args!["If you stay over 1 minute inside the ending waiting room, you will be forced outside and will not receive any arena points. Remember that."])?;
    ctx.next()?;
    ctx.lines_as("Vicious", args!["Okay, take care now."])?;
    ctx.close()
}

pub fn helper_epin(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Epin",
        args![
            "Good day,",
            "I am a helper of Lv 80 arena stage and my name is Epin.",
            "I like reading and I am 19 years old...huh...? Don't you want to hear about me...?"
        ],
    )?;
    ctx.next()?;
    ctx.npc().emotion(constants::ET_CRY)?;
    ctx.lines_as("Epin", args!["Okay..."])?;
    ctx.next()?;
    ctx.lines_as(
        "Epin",
        args![
            "This ^3131FFLv 80 arena stage^000000",
            "is accessable to characters from ^FF0000level 80^000000 to ^FF0000level 99^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Epin",
        args![
            "This level limitation is to prevent high level characters abusing low level arena stages. I hope you will understand.",
            "Also we accept an entrance fee, 1,000 zeny."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Epin", args!["Let me introduce you about the play rules of arena."])?;
    ctx.next()?;
    ctx.lines_as(
        "Epin",
        args![
            "^3131FFWait in a chat room for your turn coming.^000000",
            "When it's your turn, you will be automatically warped to an arena map."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Epin",
        args![
            "As immediately as you enter, a timer to check your play time will be activated.",
            "Please follow what ^3131FFActus^000000 guides you. "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Epin", args!["You have a ^3131FF8 minutes^000000 battle time."])?;
    ctx.next()?;
    ctx.lines_as("Epin", args!["After you clear every room including a boss room, you will be warped again to an ^3131FFending waiting room^000000.", "You can only allow to stay in the waiting room for ^3131FF1 minute^000000, please hurry up to receive the arena points and leave."])?;
    ctx.next()?;
    ctx.lines_as("Epin", args!["If you stay over 1 minute inside the ending waiting room, you will be forced outside and will not receive any arena points. Please remember that."])?;
    ctx.next()?;
    ctx.lines_as("Epin", args!["I hope you will have a good time."])?;
    ctx.close()
}

pub fn helper_lunic(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Lunic",
        args![
            "Welcome to party arena stage.",
            "I hope you will listen carefully to my introduction",
            "since this party arena stage is a little bit different from player mode stages."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "^3131FFParty Arena Stage^000000",
            "is accessible to players from ^FF0000level 10^000000 to ^FF0000level 99^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args!["Also, arena will be not be started until all of 5 party members enter the room. We accept an entrance fee 1,000 zeny."],
    )?;
    ctx.next()?;
    ctx.lines_as("Lunic", args!["Let me introduce you about the play rules of arena."])?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "^3131FFGroup a party with your friends and wait in a chat room^000000.",
            "You must form a party beforehand. If you didn't do, you could still play but you would have a lot of inconvenience."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Lunic", args!["Also please remember 5 players automatically warped to the arena room will be in order of ^FF0000entering a chat room^000000 not in order of ^FF0000party^000000. Please remember that."])?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args!["And please form a party with ^3131FF5 players^000000 before you enter a chat room."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "When it becomes your turn, you will be warped into a small map where a warp and a help NPC are located.",
            "In case you enter with members of a different party, use the warp to escape the map. Then you will return to the waiting room."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "In case you enter with your party members, please proceed speaking with the help NPC in the small map.",
            "The help NPC is only accessible to talk ^FF0000 1 player ^000000at a time."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "The NPC will guide you and your party members to the actual arena room.",
            "However, if anyone in the party ^3131FFdoes not have enough money to pay the entrance fee, he will be warped outside^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args!["Also be aware that you can only stay inside the map for ^FF0000 1 minute^000000."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "As immediately as you enter the arena map, a timer which calculates your battle time will be activated.",
            "Please follow what ^3131FFSlipslowrun^000000 guides you. "
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Lunic", args!["You will have ^3131FF10 minutes^000000 to clear the stage however it is not that easy to do. And you're advised to use the time wisely."])?;
    ctx.next()?;
    ctx.lines_as("Lunic", args!["After you clear every room including a boss room, you will be warped again to an ^3131FFending waiting room^000000.", "You can only allow to stay in the waiting room for ^3131FF1 minute^000000, please hurry up to receive the arena points and leave."])?;
    ctx.next()?;
    ctx.lines_as("Lunic", args!["If you stay over 1 minute inside the ending waiting room, you will be forced outside and will not receive any arena points. Please remember that."])?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "Besides, if a party make a new record on time to clear the map,",
            "^3131FFthe party master^000000 can record ^3131FFthe party name^000000."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args![
            "In this case, if a party has more than one master or none, it is impossible to write ^FF0000the top party record^000000.",
            "Therefore, it is strongly suggested to form one party before entering the arena map."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Lunic",
        args!["Thank you for listening and", "I hope you will have a good time."],
    )?;
    ctx.close()
}

pub fn helper_lonik(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_BEST)?;
    ctx.lines_as("Lonik", args!["Tah dah! Here I am!"])?;
    ctx.next()?;
    ctx.lines_as("Lonik", args!["You are curious if there is anyone inside or not, aren't you?"])?;
    ctx.next()?;
    ctx.lines_as("Lonik", args!["My answer is...", "............."])?;
    if ctx.call(Function::GetMapUsers, args!["force_1-2"])?.is_true() {
        ctx.npc().emotion(constants::ET_O)?;
        ctx.lines(args!["Yes!", "There is someone inside."])?;
        ctx.next()?;
        ctx.lines_as("Lonik", args!["You'd better wait a little bit longer!"])?;
    } else {
        ctx.npc().emotion(constants::ET_X)?;
        ctx.lines(args!["No!", "Go for it, good luck!"])?;
    }
    ctx.close()
}

pub fn arena_manager_arena(ctx: &Ctx) -> Script {
    let mut l_arena = Val::from(0);
    let mut l_arenamin = Val::from(0);
    let mut l_arenasec = Val::from(0);
    let mut l_i = shared::other_gm_npcs::f_gm_npc(ctx, args![1357, 0])?;
    let mut l_min_s = Val::from("");
    let mut l_mode_s = Val::from("");
    let mut l_sec_s = Val::from("");
    if l_i == -1 {
        ctx.lines_as("Arena Manager", args!["Command has been canceled."])?;
        return ctx.close();
    }
    if l_i == 0 {
        ctx.mes("Password is incorrect.")?;
        return ctx.close();
    }
    ctx.lines_as("Arena Manager", args!["Select an option."])?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from("Restart arena:Rearrange the Ranking Time")],
        )?);
        let mut matched1 = false;
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            'b2: {
                let subject2 = Val::from(runtime::select_values(ctx, &[Val::from("Lv 50:Lv 60:Lv 70:Lv 80:Party Mode")])?);
                let mut matched2 = false;
                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                    matched2 = true;
                }
                if matched2 {
                    l_arena = Val::from(50);
                    break 'b2;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                    matched2 = true;
                }
                if matched2 {
                    l_arena = Val::from(60);
                    break 'b2;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                    matched2 = true;
                }
                if matched2 {
                    l_arena = Val::from(70);
                    break 'b2;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                    matched2 = true;
                }
                if matched2 {
                    l_arena = Val::from(80);
                    break 'b2;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                    matched2 = true;
                }
                if matched2 {
                    ctx.lines_as(
                        "Arena Manager",
                        args![
                            "== Caution ==",
                            "You have chosen to restart party arena stage.",
                            "Do you wish to proceed?"
                        ],
                    )?;
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.npc().do_event("Ponox::OnStart")?;
                            ctx.lines_as("Arena Manager", args!["The arena stage has been successfuly reactivated."])?;
                            return ctx.close();
                        }
                        _ => {
                            ctx.lines_as("Arena Manager", args!["Command has been canceled."])?;
                            return ctx.close();
                        }
                    }
                }
            }
            ctx.lines_as(
                "Arena Manager",
                args![
                    "== Caution ==",
                    Val::from("You have chosen to restart Lv ") + l_arena.clone() + " arena stage.",
                    "Do you wish to proceed?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    ctx.call(
                        Function::DoNpcEvent,
                        args![Val::from("Lv") + l_arena.clone() + " Waiting Room::OnStart"],
                    )?;
                    ctx.lines_as("Arena Manager", args!["The arena stage has been successfuly reactivated."])?;
                    return ctx.close();
                }
                _ => {
                    ctx.lines_as("Arena Manager", args!["Command has been canceled."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Arena Manager", args!["== Caution ==", "^CE0000You have chosen to rearrange the ranking time. Make sure this is not a good decision unless if something serious was happened!^000000"])?;
            ctx.next()?;
            ctx.lines_as("Arena Manager", args!["Are you sure you want to rearrange the ranking time?"])?;
            ctx.next()?;
            match ctx.menu(&["No", "Yes"])? {
                0 => {
                    ctx.lines_as("Arena Manager", args!["Phew~ :)"])?;
                    return ctx.close();
                }
                _ => {
                    match ctx.menu(&["Lv 50", "Lv 60", "Lv 70", "Lv 80", "Party Mode"])? {
                        0 => {
                            l_min_s = Val::from("$top50min");
                            l_sec_s = Val::from("$top50sec");
                            l_mode_s = Val::from("Lv 50");
                        }
                        1 => {
                            l_min_s = Val::from("$top60min");
                            l_sec_s = Val::from("$top60sec");
                            l_mode_s = Val::from("Lv 60");
                        }
                        2 => {
                            l_min_s = Val::from("$top70min");
                            l_sec_s = Val::from("$top70sec");
                            l_mode_s = Val::from("Lv 70");
                        }
                        3 => {
                            l_min_s = Val::from("$top80min");
                            l_sec_s = Val::from("$top80sec");
                            l_mode_s = Val::from("Lv 80");
                        }
                        _ => {
                            l_min_s = Val::from("$top_ptmin");
                            l_sec_s = Val::from("$top_ptsec");
                            l_mode_s = Val::from("party");
                        }
                    }
                    ctx.lines_as(
                        "Arena Manager",
                        args!["Please enter a value for minutes first and then seconds."],
                    )?;
                    ctx.next()?;
                    let (input, _) = runtime::input_number(ctx, None, None)?;
                    l_arenamin = input;
                    runtime::setd(
                        ctx,
                        &l_min_s.clone(),
                        l_arenamin.clone(),
                        &mut [
                            (".@arena", runtime::LocalMut::Scalar(&mut l_arena)),
                            (".@arenamin", runtime::LocalMut::Scalar(&mut l_arenamin)),
                            (".@arenasec", runtime::LocalMut::Scalar(&mut l_arenasec)),
                            (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                            (".@min$", runtime::LocalMut::Scalar(&mut l_min_s)),
                            (".@mode$", runtime::LocalMut::Scalar(&mut l_mode_s)),
                            (".@sec$", runtime::LocalMut::Scalar(&mut l_sec_s)),
                        ],
                    )?;
                    let (input, _) = runtime::input_number(ctx, None, None)?;
                    l_arenasec = input;
                    runtime::setd(
                        ctx,
                        &l_sec_s.clone(),
                        l_arenasec.clone(),
                        &mut [
                            (".@arena", runtime::LocalMut::Scalar(&mut l_arena)),
                            (".@arenamin", runtime::LocalMut::Scalar(&mut l_arenamin)),
                            (".@arenasec", runtime::LocalMut::Scalar(&mut l_arenasec)),
                            (".@i", runtime::LocalMut::Scalar(&mut l_i)),
                            (".@min$", runtime::LocalMut::Scalar(&mut l_min_s)),
                            (".@mode$", runtime::LocalMut::Scalar(&mut l_mode_s)),
                            (".@sec$", runtime::LocalMut::Scalar(&mut l_sec_s)),
                        ],
                    )?;
                    ctx.lines_as(
                        "Arena Manager",
                        args![
                            Val::from("Current ")
                                + l_mode_s.clone()
                                + " ranker's play time has been rearranged to ^FF0000"
                                + runtime::getd(
                                    ctx,
                                    &l_min_s.clone(),
                                    &[
                                        (".@arena", runtime::Local::Scalar(&l_arena)),
                                        (".@arenamin", runtime::Local::Scalar(&l_arenamin)),
                                        (".@arenasec", runtime::Local::Scalar(&l_arenasec)),
                                        (".@i", runtime::Local::Scalar(&l_i)),
                                        (".@min$", runtime::Local::Scalar(&l_min_s)),
                                        (".@mode$", runtime::Local::Scalar(&l_mode_s)),
                                        (".@sec$", runtime::Local::Scalar(&l_sec_s))
                                    ]
                                )?
                                + "^000000 minutes and ^FF0000"
                                + runtime::getd(
                                    ctx,
                                    &l_sec_s.clone(),
                                    &[
                                        (".@arena", runtime::Local::Scalar(&l_arena)),
                                        (".@arenamin", runtime::Local::Scalar(&l_arenamin)),
                                        (".@arenasec", runtime::Local::Scalar(&l_arenasec)),
                                        (".@i", runtime::Local::Scalar(&l_i)),
                                        (".@min$", runtime::Local::Scalar(&l_min_s)),
                                        (".@mode$", runtime::Local::Scalar(&l_mode_s)),
                                        (".@sec$", runtime::Local::Scalar(&l_sec_s))
                                    ]
                                )?
                                + "^000000 seconds."
                        ],
                    )?;
                    return ctx.close();
                }
            }
        }
    }
    Ok(())
}

pub fn reward_manager_arena(ctx: &Ctx) -> Script {
    let l_i = shared::other_gm_npcs::f_gm_npc(ctx, args![1357, 0])?;
    if l_i == -1 {
        ctx.lines_as("Reward Manager", args!["Command has been canceled."])?;
        return ctx.close();
    }
    if l_i == 0 {
        ctx.lines_as("Reward Manager", args!["Password is incorrect."])?;
        return ctx.close();
    }
    ctx.lines_as("Reward Manager", args!["You have chosen to hide the teleporter NPC."])?;
    ctx.next()?;
    match ctx.menu(&["Cancel", "Yes", "Turn on"])? {
        0 => {
            ctx.lines_as("Reward Manager", args!["You have canceled the command."])?;
            ctx.close()
        }
        1 => {
            ctx.lines_as("Reward Manager", args!["NPC has been hidden."])?;
            ctx.set_npc_visible("Teleporter#arena", false)?;
            ctx.close()
        }
        _ => {
            ctx.lines_as("Reward Manager", args!["NPC has been enabled."])?;
            ctx.set_npc_visible("Teleporter#arena", true)?;
            ctx.close()
        }
    }
}

pub fn teleporter_arena(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Teleporter",
        args!["I can move you to the reward arena!", "Would you like to go there?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Yes", "No."])? {
        0 => {
            ctx.lines_as("Teleporter", args!["Let me guide you."])?;
            ctx.close_window()?;
            ctx.warp("prt_are_in", 60, 14)?;
            ctx.end()
        }
        _ => {
            ctx.lines_as("Teleporter", args!["No problem, feel free to come back any time."])?;
            ctx.close()
        }
    }
}

pub fn givu_arena(ctx: &Ctx) -> Script {
    if ctx.call(Function::CheckWeight, args![526, 5])? == 0 {
        ctx.lines(args![
            "- Wait a moment! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please lighten your weight -",
            "- and try again. -"
        ])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Givu",
        args![
            "Hello, there. Welcome to the world of Arena.",
            "My name is Givu, I am in charge of arena point exchange program."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Givu",
        args![
            "You can exchange your arena points with various stuffs.",
            "Please choose a menu below."
        ],
    )?;
    ctx.next()?;
    'b1: {
        let subject1 = Val::from(runtime::select_values(
            ctx,
            &[Val::from(
                "Exchange with Consumable items:Exchange with EXP points:Take a Souvenir Picture:Check Current Arena Points",
            )],
        )?);
        let mut matched1 = false;
        if !matched1 && subject1.loosely_equals(&Val::from(1)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Givu", args!["What consumable item do you wish to exchange?"])?;
            ctx.next()?;
            'b2: {
                let subject2 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Honey:Royal Jelly:Mastela Fruit:Condensed White Potion:Anodyne:Yggdrasil Seed:Yggdrasilberry:Old Blue Box:Old Purple Box:Old Card Album",
                    )],
                )?);
                let mut matched2 = false;
                if !matched2 && subject2.loosely_equals(&Val::from(1)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![518, 5, 20])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(2)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![526, 5, 30])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(3)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![522, 5, 30])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(4)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![547, 8, 30])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(5)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![605, 3, 20])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(6)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![608, 1, 20])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(7)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![607, 1, 40])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(8)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![603, 1, 100])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(9)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![617, 1, 300])?;
                }
                if !matched2 && subject2.loosely_equals(&Val::from(10)) {
                    matched2 = true;
                }
                if matched2 {
                    shared::other_arena_arena_room::func_are_rew(ctx, args![616, 1, 1000])?;
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(2)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Givu",
                args![
                    "Would you like to exchange your arena points with experience points?",
                    "It requires 40 arena points."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Cancel", "Yes"])? {
                0 => {
                    ctx.lines_as("Givu", args!["You have canceled your request."])?;
                    return ctx.close();
                }
                _ => {
                    if ctx.var("arena_point").get()?.number()? < 40 {
                        ctx.lines_as(
                            "Givu",
                            args![
                                "You do not have enough arena points.",
                                "Please check the total amount of arena points you have."
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.var("arena_point").set(ctx.var("arena_point").get()?.number()? - 40)?;
                    if ctx.player().base_level()? < 70 {
                        ctx.call(Function::GetExperience, args![3000, 0])?;
                    } else if ctx.player().base_level()? < 80 {
                        ctx.call(Function::GetExperience, args![9000, 0])?;
                    } else if ctx.player().base_level()? < 90 {
                        ctx.call(Function::GetExperience, args![10000, 0])?;
                    } else {
                        ctx.call(Function::GetExperience, args![30000, 0])?;
                    }
                    ctx.lines_as("Givu", args!["You have gained experience points. Thank you."])?;
                    return ctx.close();
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(3)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as("Givu", args!["You have chosen a souvenir picture services."])?;
            ctx.next()?;
            'b4: {
                let subject4 = Val::from(runtime::select_values(
                    ctx,
                    &[Val::from("About souvenir Picture Services:Take a picture")],
                )?);
                let mut matched4 = false;
                if !matched4 && subject4.loosely_equals(&Val::from(1)) {
                    matched4 = true;
                }
                if matched4 {
                    ctx.lines_as("Givu", args!["Do you see stairs at the right side of me?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Givu",
                        args!["At the stairs, you can take a screenshot with an NPC or a monster."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Givu",
                        args!["When you choose an NPC or a monster, it will show some emotion icons ^FF0000for 1 minute^000000."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Givu", args!["Don't miss the chance to take a picture with your favorite NPC!"])?;
                    return ctx.close();
                }
                if !matched4 && subject4.loosely_equals(&Val::from(2)) {
                    matched4 = true;
                }
                if matched4 {
                    if ctx.var("$@arena_picture").get()?.is_true() {
                        ctx.lines_as("Givu", args!["A souvenir picture services is on progress. Please wait."])?;
                        return ctx.close();
                    }
                    ctx.lines_as(
                        "Givu",
                        args![
                            "Please choose an NPC at below.",
                            "The NPC will appear for 1 minute and this service requires 10 arena points."
                        ],
                    )?;
                    ctx.next()?;
                    let picture_id: i32 = match ctx.menu(&[
                        "Baphomet",
                        "Dark Lord",
                        "Doppelganger",
                        "Eddga",
                        "Dracula",
                        "Samurai",
                        "Stormy Knight",
                        "Phreeoni",
                        "Girl",
                        "Valkyrie",
                    ])? {
                        0 => 1039,
                        1 => 1272,
                        2 => 1046,
                        3 => 1115,
                        4 => 1389,
                        5 => 1492,
                        6 => 1251,
                        7 => 1159,
                        8 => 6969,
                        _ => 7777,
                    };
                    ctx.var("$@arena_picture_id").set(Val::from(picture_id))?;
                    if ctx.var("arena_point").get()?.number()? < 10 {
                        ctx.lines_as(
                            "Givu",
                            args![
                                "You do not have enough arena points.",
                                "Please check the total amount of arena points you have."
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.mes("[Givu]")?;
                    if ctx.var("$@arena_picture_id").get()? == 6969 {
                        ctx.mes("Would you like to take a picture with pretty girls?")?;
                    } else if ctx.var("$@arena_picture_id").get()? == 7777 {
                        ctx.mes("Would you like to take a picture with a Valkyrie?")?;
                    } else {
                        ctx.lines(args![
                            Val::from("Would you like to take a picture with a ")
                                + ctx.call(
                                    Function::GetMonsterInfo,
                                    args![ctx.var("$@arena_picture_id").get()?, constants::MOB_NAME]
                                )?
                                + "?"
                        ])?;
                    }
                    ctx.next()?;
                    match ctx.menu(&["Yes", "No"])? {
                        0 => {
                            ctx.var("arena_point").set(ctx.var("arena_point").get()?.number()? - 10)?;
                            ctx.var("$@arena_picture").set(Val::from(1))?;
                            ctx.call(
                                Function::EnableNpc,
                                args![Val::from("#arena_") + ctx.var("$@arena_picture_id").get()?],
                            )?;
                            ctx.call(
                                Function::MoveNpc,
                                args![Val::from("#arena_") + ctx.var("$@arena_picture_id").get()?, 96, 28],
                            )?;
                            if ctx.var("$@arena_picture_id").get()? == 6969 {
                                ctx.set_npc_visible("#arena_ss_2", true)?;
                                ctx.set_npc_visible("#arena_ss_3", true)?;
                                ctx.set_npc_visible("#arena_ss_4", true)?;
                            }
                            ctx.npc().do_event("npctime#arena::OnStart")?;
                            ctx.lines_as("Givu", args!["Thank you."])?;
                            return ctx.close();
                        }
                        _ => {
                            ctx.lines_as("Givu", args!["Would you like to consider a little longer?"])?;
                            return ctx.close();
                        }
                    }
                }
            }
        }
        if !matched1 && subject1.loosely_equals(&Val::from(4)) {
            matched1 = true;
        }
        if matched1 {
            ctx.lines_as(
                "Givu",
                args![Val::from("Let me check ") + ctx.player().name()? + "'s current arena points."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Givu",
                args![Val::from("") + ctx.player().name()? + " has total ^3131FF" + ctx.var("arena_point").get()? + "^000000 points."],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn arena_1039(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn arena_1039_oninit(ctx: &Ctx) -> Script {
    for npc in [
        "#arena_6969",
        "#arena_ss_2",
        "#arena_ss_3",
        "#arena_ss_4",
        "#arena_1272",
        "#arena_1046",
        "#arena_1115",
        "#arena_1389",
        "#arena_1492",
        "#arena_1251",
        "#arena_1159",
        "#arena_7777",
        "#arena_1039",
    ] {
        ctx.set_npc_visible(npc, false)?;
    }
    ctx.end()
}

pub fn arena_1039_onheart(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_THROB)?;
    ctx.end()
}

pub fn arena_1039_onsci(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_SCISSOR)?;
    ctx.end()
}

pub fn arena_1039_onbest(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_BEST)?;
    ctx.end()
}

pub fn arena_1039_onomg(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_HUK)?;
    ctx.end()
}

pub fn arena_1039_onkik(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_KIK)?;
    ctx.end()
}

pub fn arena_1039_onkis(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_CHUP)?;
    ctx.end()
}

#[derive(Clone, Copy, Debug)]
enum NpctimeArenaStep {
    Start,
    OnStop,
    OnStart,
    OnTimer2000,
    OnTimer5000,
    OnTimer7000,
    OnTimer8000,
    OnTimer9000,
    OnTimer10000,
    OnTimer15000,
    OnTimer17000,
    OnTimer18000,
    OnTimer19000,
    OnTimer20000,
    OnTimer25000,
    OnTimer27000,
    OnTimer28000,
    OnTimer29000,
    OnTimer30000,
    OnTimer35000,
    OnTimer37000,
    OnTimer38000,
    OnTimer39000,
    OnTimer40000,
    OnTimer45000,
    OnTimer47000,
    OnTimer48000,
    OnTimer49000,
    OnTimer50000,
    OnTimer55000,
    OnTimer57000,
    OnTimer58000,
    OnTimer59000,
    OnTimer60000,
    OnTimer62000,
}

fn npctime_announce(ctx: &Ctx, message: &str) -> Script {
    ctx.call(Function::MapAnnounce, args!["prt_are_in", message, 1, 16764416])?;
    Ok(())
}

fn npctime_picture_event(ctx: &Ctx, event: &str) -> Script {
    ctx.call(
        Function::DoNpcEvent,
        args![Val::from("#arena_") + ctx.var("$@arena_picture_id").get()? + "::" + event],
    )?;
    if ctx.var("$@arena_picture_id").get()? == 6969 {
        for ss in ["#arena_ss_2", "#arena_ss_3", "#arena_ss_4"] {
            ctx.call(Function::DoNpcEvent, args![format!("{ss}::{event}")])?;
        }
    }
    Ok(())
}

fn npctime_arena_run(ctx: &Ctx, mut step: NpctimeArenaStep) -> Script {
    'machine: loop {
        match step {
            NpctimeArenaStep::Start => {
                step = NpctimeArenaStep::OnStop;
                continue 'machine;
            }
            NpctimeArenaStep::OnStop => {
                ctx.call(Function::StopNpcTimer, args![])?;
                return ctx.end();
            }
            NpctimeArenaStep::OnStart => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer2000 => {
                npctime_announce(ctx, "Givu : You should keep time with popping emotion icons~")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer5000 => {
                npctime_announce(ctx, "Emoticon : /lv")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer7000 => {
                npctime_announce(ctx, " 3 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer8000 => {
                npctime_announce(ctx, " 2 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer9000 => {
                npctime_announce(ctx, " 1 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer10000 => {
                npctime_announce(ctx, " ")?;
                npctime_picture_event(ctx, "OnHeart")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer15000 => {
                npctime_announce(ctx, "Emoticon : /gawi")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer17000 => {
                npctime_announce(ctx, " 3 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer18000 => {
                npctime_announce(ctx, " 2 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer19000 => {
                npctime_announce(ctx, " 1 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer20000 => {
                npctime_announce(ctx, " ")?;
                npctime_picture_event(ctx, "OnSci")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer25000 => {
                npctime_announce(ctx, "Emoticon : /no1")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer27000 => {
                npctime_announce(ctx, " 3 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer28000 => {
                npctime_announce(ctx, " 2 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer29000 => {
                npctime_announce(ctx, " 1 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer30000 => {
                npctime_announce(ctx, " ")?;
                npctime_picture_event(ctx, "OnBest")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer35000 => {
                npctime_announce(ctx, "Emoticon : /huk")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer37000 => {
                npctime_announce(ctx, " 3 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer38000 => {
                npctime_announce(ctx, " 2 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer39000 => {
                npctime_announce(ctx, " 1 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer40000 => {
                npctime_announce(ctx, " ")?;
                npctime_picture_event(ctx, "OnOmg")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer45000 => {
                npctime_announce(ctx, "Emoticon : /gg")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer47000 => {
                npctime_announce(ctx, " 3 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer48000 => {
                npctime_announce(ctx, " 2 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer49000 => {
                npctime_announce(ctx, " 1 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer50000 => {
                npctime_announce(ctx, " ")?;
                npctime_picture_event(ctx, "OnKik")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer55000 => {
                npctime_announce(ctx, "Emoticon : /kis")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer57000 => {
                npctime_announce(ctx, " 3 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer58000 => {
                npctime_announce(ctx, " 2 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer59000 => {
                npctime_announce(ctx, " 1 ")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer60000 => {
                npctime_announce(ctx, " ")?;
                npctime_picture_event(ctx, "OnKis")?;
                return ctx.end();
            }
            NpctimeArenaStep::OnTimer62000 => {
                npctime_announce(ctx, "Time is over. Thank you for using my services.")?;
                ctx.call(
                    Function::MoveNpc,
                    args![Val::from("#arena_") + ctx.var("$@arena_picture_id").get()?, 1, 1],
                )?;
                ctx.npc().do_event("#arena_1039::OnInit")?;
                ctx.var("$@arena_picture").set(Val::from(0))?;
                ctx.call(Function::StopNpcTimer, args![])?;
                return ctx.end();
            }
        }
    }
}

pub fn npctime_arena(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::Start)
}

pub fn npctime_arena_onstop(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnStop)
}

pub fn npctime_arena_onstart(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnStart)
}

pub fn npctime_arena_ontimer2000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer2000)
}

pub fn npctime_arena_ontimer5000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer5000)
}

pub fn npctime_arena_ontimer7000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer7000)
}

pub fn npctime_arena_ontimer8000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer8000)
}

pub fn npctime_arena_ontimer9000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer9000)
}

pub fn npctime_arena_ontimer10000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer10000)
}

pub fn npctime_arena_ontimer15000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer15000)
}

pub fn npctime_arena_ontimer17000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer17000)
}

pub fn npctime_arena_ontimer18000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer18000)
}

pub fn npctime_arena_ontimer19000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer19000)
}

pub fn npctime_arena_ontimer20000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer20000)
}

pub fn npctime_arena_ontimer25000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer25000)
}

pub fn npctime_arena_ontimer27000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer27000)
}

pub fn npctime_arena_ontimer28000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer28000)
}

pub fn npctime_arena_ontimer29000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer29000)
}

pub fn npctime_arena_ontimer30000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer30000)
}

pub fn npctime_arena_ontimer35000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer35000)
}

pub fn npctime_arena_ontimer37000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer37000)
}

pub fn npctime_arena_ontimer38000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer38000)
}

pub fn npctime_arena_ontimer39000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer39000)
}

pub fn npctime_arena_ontimer40000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer40000)
}

pub fn npctime_arena_ontimer45000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer45000)
}

pub fn npctime_arena_ontimer47000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer47000)
}

pub fn npctime_arena_ontimer48000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer48000)
}

pub fn npctime_arena_ontimer49000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer49000)
}

pub fn npctime_arena_ontimer50000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer50000)
}

pub fn npctime_arena_ontimer55000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer55000)
}

pub fn npctime_arena_ontimer57000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer57000)
}

pub fn npctime_arena_ontimer58000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer58000)
}

pub fn npctime_arena_ontimer59000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer59000)
}

pub fn npctime_arena_ontimer60000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer60000)
}

pub fn npctime_arena_ontimer62000(ctx: &Ctx) -> Script {
    npctime_arena_run(ctx, NpctimeArenaStep::OnTimer62000)
}

pub fn picture_manager_arena(ctx: &Ctx) -> Script {
    let l_i = shared::other_gm_npcs::f_gm_npc(ctx, args![1357, 0])?;
    if l_i == -1 {
        ctx.lines_as("Picture Manager", args!["Command has been canceled."])?;
        return ctx.close();
    }
    if l_i == 0 {
        ctx.lines_as("Picture Manager", args!["Password is incorrect."])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Picture Manager",
        args!["Would you like to reset Picture Reward variable to 0?"],
    )?;
    ctx.next()?;
    match ctx.menu(&["Cancel", "Yes"])? {
        0 => {
            ctx.lines_as("Picture Manager", args!["You have canceled your request."])?;
            ctx.close()
        }
        _ => {
            ctx.lines_as("Picture Manager", args!["Picture rewarding reseted."])?;
            ctx.var("$@arena_picture").set(Val::from(0))?;
            ctx.close()
        }
    }
}

pub fn live_broadcast_arena(ctx: &Ctx) -> Script {
    shared::other_gm_npcs::f_gm_npc(ctx, args![])?;
    ctx.lines_as(
        "Live Broadcast",
        args![
            Val::from("Currently there are ") + ctx.call(Function::GetMapUsers, args!["force_1-1"])? + " people in Lv 50s map.",
            Val::from("Currently there are ") + ctx.call(Function::GetMapUsers, args!["force_2-1"])? + " people in Lv 60s map.",
            Val::from("Currently there are ") + ctx.call(Function::GetMapUsers, args!["force_3-1"])? + " people in Lv 70s map.",
            Val::from("Currently there are ") + ctx.call(Function::GetMapUsers, args!["force_4-1"])? + " people in Lv 80s map.",
            Val::from("Currently there are ") + ctx.call(Function::GetMapUsers, args!["force_1-2"])? + " people in party map.",
            Val::from("Currently there are ") + ctx.call(Function::GetMapUsers, args!["arena_room"])? + " people in the waiting room."
        ],
    )?;
    ctx.close()
}
