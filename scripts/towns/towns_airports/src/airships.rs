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
enum Airshipwarp1Step {
    Start,
    OnInit,
    OnHide,
    OnUnhide,
    OnTouch,
}

fn airshipwarp_1_run(ctx: &Ctx, mut step: Airshipwarp1Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Airshipwarp1Step::Start => {
                return Err(Stop::End);
            }
            Airshipwarp1Step::OnInit => {
                step = Airshipwarp1Step::OnHide;
                continue 'machine;
            }
            Airshipwarp1Step::OnHide => {
                ctx.npc().special_effect(constants::EF_BASH)?;
                ctx.call(Function::DisableNpc, args![])?;
                return Err(Stop::End);
            }
            Airshipwarp1Step::OnUnhide => {
                ctx.call(Function::EnableNpc, args![])?;
                ctx.npc().special_effect(constants::EF_SUMMONSLAVE)?;
                return Err(Stop::End);
            }
            Airshipwarp1Step::OnTouch => {
                let subject1 = ctx.var("$@airplanelocation").get()?;
                if subject1 == 0 {
                    ctx.warp("yuno", 92, 260)?;
                    return Err(Stop::End);
                } else if subject1 == 1 {
                    ctx.warp("einbroch", 92, 278)?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.warp("lighthalzen", 302, 75)?;
                    return Err(Stop::End);
                } else if subject1 == 3 {
                    ctx.warp("hugel", 181, 146)?;
                    return Err(Stop::End);
                }
                return Ok(Val::from(0));
            }
        }
    }
}

pub fn airshipwarp_1(ctx: &Ctx) -> Script {
    airshipwarp_1_run(ctx, Airshipwarp1Step::Start, Vec::new()).map(|_| ())
}

pub fn airshipwarp_1_oninit(ctx: &Ctx) -> Script {
    airshipwarp_1_run(ctx, Airshipwarp1Step::OnInit, Vec::new()).map(|_| ())
}

pub fn airshipwarp_1_onhide(ctx: &Ctx) -> Script {
    airshipwarp_1_run(ctx, Airshipwarp1Step::OnHide, Vec::new()).map(|_| ())
}

pub fn airshipwarp_1_onunhide(ctx: &Ctx) -> Script {
    airshipwarp_1_run(ctx, Airshipwarp1Step::OnUnhide, Vec::new()).map(|_| ())
}

pub fn airshipwarp_1_ontouch(ctx: &Ctx) -> Script {
    airshipwarp_1_run(ctx, Airshipwarp1Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn domestic_airship(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn domestic_airship_oninit(ctx: &Ctx) -> Script {
    ctx.call(Function::InitNpcTimer, args![])?;
    ctx.end()
}

pub fn domestic_airship_ontimer20000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We are heading to Einbroch.", constants::BC_MAP, "0x00ff00"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer50000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We will arrive in Einbroch shortly.", constants::BC_MAP, "0x00ff00"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer60000(ctx: &Ctx) -> Script {
    ctx.var("$@airplanelocation").set(Val::from(1))?;
    ctx.npc().do_event("#AirshipWarp-1::OnUnhide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnUnhide")?;
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "Welcome to Einbroch. Have a safe trip.", constants::BC_MAP, "0x00ff00"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer70000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "Currently we are in Einbroch. The Airship will take off shortly.",
            constants::BC_MAP,
            "0x00ff00"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer80000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("#AirshipWarp-1::OnHide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnHide")?;
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "The Airship is now taking off. Our next destination is Lighthalzen.",
            constants::BC_MAP,
            "0x70dbdb"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer100000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We are heading to Lighthalzen.", constants::BC_MAP, "0x70dbdb"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer130000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We will arrive in Lighthalzen shortly.", constants::BC_MAP, "0x70dbdb"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer140000(ctx: &Ctx) -> Script {
    ctx.var("$@airplanelocation").set(Val::from(2))?;
    ctx.npc().do_event("#AirshipWarp-1::OnUnhide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnUnhide")?;
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "Welcome to Lighthalzen. Have a safe trip.",
            constants::BC_MAP,
            "0x70dbdb"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer150000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "Currently we are in Lighthalzen. The Airship will leave shortly.",
            constants::BC_MAP,
            "0x70dbdb"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer160000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("#AirshipWarp-1::OnHide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnHide")?;
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "The Airship is leaving the ground. Our next destination is Einbroch.",
            constants::BC_MAP,
            "0x00ff00"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer180000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We are heading to Einbroch.", constants::BC_MAP, "0x00ff00"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer210000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We will arrive in Einbroch shortly.", constants::BC_MAP, "0x00FF00"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer220000(ctx: &Ctx) -> Script {
    ctx.var("$@airplanelocation").set(Val::from(1))?;
    ctx.npc().do_event("#AirshipWarp-1::OnUnhide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnUnhide")?;
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "Welcome to Einbroch. Have a safe trip.", constants::BC_MAP, "0x00ff00"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer230000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "Currently we are in Einbroch. The Airship will take off shortly.",
            constants::BC_MAP,
            "0x00ff00"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer240000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("#AirshipWarp-1::OnHide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnHide")?;
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "The Airship is now taking off. Our next destination is Juno.",
            constants::BC_MAP,
            "0xff8200"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer260000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We are heading to Juno.", constants::BC_MAP, "0xff8200"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer290000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We will arrive in Juno shortly.", constants::BC_MAP, "0xff8200"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer300000(ctx: &Ctx) -> Script {
    ctx.var("$@airplanelocation").set(Val::from(0))?;
    ctx.npc().do_event("#AirshipWarp-1::OnUnhide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnUnhide")?;
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "Welcome to Juno. Have a safe trip.", constants::BC_MAP, "0xff8200"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer310000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "Currently we are in Juno. The Airship will leave shortly.",
            constants::BC_MAP,
            "0xff8200"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer320000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("#AirshipWarp-1::OnHide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnHide")?;
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "The Airship is leaving the ground. Our next destination is Hugel.",
            constants::BC_MAP,
            "0xca4bf3"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer340000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We are heading to Hugel.", constants::BC_MAP, "0xca4bf3"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer370000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We will arrive in Hugel shortly.", constants::BC_MAP, "0xca4bf3"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer380000(ctx: &Ctx) -> Script {
    ctx.var("$@airplanelocation").set(Val::from(3))?;
    ctx.npc().do_event("#AirshipWarp-1::OnUnhide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnUnhide")?;
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "Welcome to Hugel. Have a safe trip.", constants::BC_MAP, "0xca4bf3"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer390000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "Currently we are in Hugel. The Airship will leave shortly.",
            constants::BC_MAP,
            "0xca4bf3"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer400000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("#AirshipWarp-1::OnHide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnHide")?;
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "The Airship is leaving the ground. Our next destination is Juno.",
            constants::BC_MAP,
            "0xff8200"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer420000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We are heading to Juno.", constants::BC_MAP, "0xff8200"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer450000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "We will arrive in Juno shortly.", constants::BC_MAP, "0xff8200"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer460000(ctx: &Ctx) -> Script {
    ctx.var("$@airplanelocation").set(Val::from(0))?;
    ctx.npc().do_event("#AirshipWarp-1::OnUnhide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnUnhide")?;
    ctx.call(
        Function::MapAnnounce,
        args!["airplane", "Welcome to Juno. Have a safe trip.", constants::BC_MAP, "0xff8200"],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer470000(ctx: &Ctx) -> Script {
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "Currently we are in Juno. The Airship will leave shortly.",
            constants::BC_MAP,
            "0xff8200"
        ],
    )?;
    ctx.end()
}

pub fn domestic_airship_ontimer480000(ctx: &Ctx) -> Script {
    ctx.npc().do_event("#AirshipWarp-1::OnHide")?;
    ctx.npc().do_event("#AirshipWarp-2::OnHide")?;
    ctx.call(
        Function::MapAnnounce,
        args![
            "airplane",
            "The Airship is leaving the ground. Our next destination is Einbroch.",
            constants::BC_MAP,
            "0x00ff00"
        ],
    )?;
    ctx.call(Function::StopNpcTimer, args![])?;
    ctx.call(Function::InitNpcTimer, args![])?;
    Ok(())
}

pub fn exit_airplane1a(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn airship_crew_ein_1(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Airship Crew",
        args![
            "If we've landed at",
            "your destination and",
            "you'd like to leave the",
            "Airship, please use the",
            "stairs up ahead. Thank",
            "you for your patronage."
        ],
    )?;
    ctx.close()
}

pub fn umbala_kid_ein_p(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_PROFUSELY_SWEAT)?;
    ctx.mes("[Kid]")?;
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.lines(args![
            "Wow, mom!",
            "L-look at this!",
            "We're flying! W-we're...",
            "We're in the freakin' sky!"
        ])?;
    } else {
        ctx.lines(args![
            "Makumalagu!",
            "Saampa joojimbo",
            "kaku na jedi Solo.",
            "Bwahahahahahahaah!"
        ])?;
    }
    ctx.close()
}

pub fn umbala_lady_ein_p(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_THINK)?;
    ctx.mes("[Lady]")?;
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.lines(args![
            "Shush...",
            "Honey, behave~",
            "Don't act so excited",
            "when we're out in a",
            "public place like this!"
        ])?;
    } else {
        ctx.lines(args!["Chooktu!", "Sacraup matii!", "Shaka gurftalfi", "huntiki manjoo!"])?;
    }
    ctx.close()
}

pub fn umbala_man_ein_p(ctx: &Ctx) -> Script {
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.lines_as(
            "Chrmlim",
            args![
                "Hey there~",
                "From that look on",
                "your face, I see that",
                "you can understand",
                "me. ^333333*Whew...!*^000000"
            ],
        )?;
        ctx.next()?;
        ctx.npc().emotion(constants::ET_HNG)?;
        ctx.lines_as(
            "Chrmlim",
            args![
                "I've been helping the",
                "Airship enterprise by",
                "having the Airship Crewmen",
                "train in Umbala to overcome",
                "any acrophobia they might have through bungee jumping. Neat, eh?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chrmlim",
            args![
                "But...",
                "Some of them couldn't",
                "overcome their fear of",
                "heights. And a few even",
                "ended up, um, ^333333in Niflheim^000000."
            ],
        )?;
    } else {
        ctx.lines_as("Chrmlim", args!["Bajoo ga", "nukta Airship."])?;
        ctx.next()?;
        ctx.lines_as("Chrmlim", args!["...", "......"])?;
        ctx.next()?;
        ctx.npc().emotion(constants::ET_HNG)?;
        ctx.lines_as(
            "Chrmlim",
            args![
                "Shabala moow bajama",
                "Airship kulaha googoona ",
                "salu. Dama, kookoo na nu",
                "yukuta. Um, fashuku na ret!"
            ],
        )?;
    }
    ctx.close()
}

pub fn airship_staff_airplane(ctx: &Ctx) -> Script {
    if ctx.var("hg_ma1").get()? == 3 {
        ctx.lines_as("Airship Staff", args!["Welcome", "to the Airship.", "How may I help you?"])?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Do you have a passenger named Thierry?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Airship Staff",
            args!["I am sorry, but I do not think that we have a passenger by that name."],
        )?;
        return ctx.close();
    }
    ctx.lines_as("Airship Staff", args!["Welcome", "to the Airship.", "How may I help you?"])?;
    ctx.next()?;
    match ctx.menu(&["Using the Airship", "Captain's Cabin", "Facilities", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "When you see a broadcast",
                    "announcing that we have",
                    "arrived at your destination,",
                    "please use one of the exits",
                    "located at the north and",
                    "south ends of the Airship."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Airship Staff",
                args![
                    "If you happen to miss",
                    "your stop, don't worry.",
                    "The Airship is constantly",
                    "en route and you'll get",
                    "another chance to arrive",
                    "to your intended destination."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "The Captain's Cabin",
                    "is located at the front",
                    "of the Airship. There, you",
                    "can meet the captain and",
                    "the pilot of the Airship."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "The Airship provides",
                    "various Mini Games for",
                    "the entertainment of all",
                    "our passengers. We invite",
                    "you to try your luck and skills",
                    "in the Airship's Mini Games~"
                ],
            )?;
            return ctx.close();
        }
        _ => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "Well, I hope you",
                    "your flight aboard",
                    "our Airships. Thank",
                    "you and have a good day."
                ],
            )?;
            return ctx.close();
        }
    }
}

pub fn zerta_01airplane(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Zerta",
        args![
            "Oh, hello adventurer.",
            "I am currently on a",
            "sacred journey, offering",
            "prayer for the sake of the",
            "Midgard continent."
        ],
    )?;
    ctx.close()
}

pub fn maelin_01airplane(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Maelin",
        args![
            "Um, this Airship is",
            "to Lutie, isn't it? I've",
            "waiting so long,",
            "but I haven't heard any",
            "broadcast about Lutie."
        ],
    )?;
    ctx.close()
}

pub fn aanos_01airplane(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Aanos",
        args!["Oh wooow~", "The sky looks", "so different and", "pretty from up there!"],
    )?;
    ctx.close()
}

pub fn pilot_airplane(ctx: &Ctx) -> Script {
    if ctx.var("hg_ma1").get()? == 3 {
        ctx.lines_as(
            "Pilot",
            args![
                "I wish that I could go drink a cold fresh beer.",
                "Drinking is the goal of my life! Drinking gives me energy!",
                "I am nothing without drinks!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pilot",
            args![
                "But! Driving under the influence is not good.",
                "But! That makes me want to drink more and more!"
            ],
        )?;
        ctx.npc().emotion(constants::ET_CRY)?;
        ctx.next()?;
        let choice = runtime::select_values(ctx, &[Val::from("Do you know a passenger named Thierry?")])?;
        ctx.var("@menu").set(choice)?;
        ctx.lines_as(
            "Pilot",
            args![
                "This uniform is",
                "really dapper, but",
                "it's way too thick to",
                "wear around the Airship."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Pilot",
            args![
                "...",
                "......",
                "No one ever really",
                "comes into this room.",
                "And the captain IS a reindeer. I could just strip to my boxers."
            ],
        )?;
        ctx.next()?;
        ctx.npc().emotion(constants::ET_HUK)?;
        ctx.lines_as("Pilot", args!["Wah!? Who is it!"])?;
        ctx.next()?;
        ctx.mes("- ...He is not listening to you, at all. -")?;
        return ctx.close();
    }
    match ctx.rand_range(1, 4)? {
        1 => {
            ctx.lines_as(
                "Pilot",
                args![
                    "It's been sooo",
                    "long since I've",
                    "enjoyed a nice, cold",
                    "alcoholic brew. But the",
                    "job requires me to be as",
                    "clear headed as I can!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pilot",
                args![
                    "Always drink responsibly!",
                    "Still, I can't remember the",
                    "last time I had a real vacation",
                    "or even a day off. Yeap, some",
                    "booze, some chips, some TV",
                    "and serious R&R is in order."
                ],
            )?;
            ctx.npc().emotion(constants::ET_CRY)?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Pilot",
                args![
                    "Man, the weather",
                    "is really nice today.",
                    "Bright, open skies make",
                    "for some good visibility",
                    "and safe, carefree flying."
                ],
            )?;
            return ctx.close();
        }
        3 => {
            ctx.lines_as(
                "Pilot",
                args![
                    "You know, our captain's a",
                    "respectable guy. Him and",
                    "his brother are actually well",
                    "known in the aircraft industry.",
                    "Who knew reindeer made",
                    "such good captains?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pilot",
                args![
                    "Just between you",
                    "and me, I gotta tell",
                    "you, that Santa was onto",
                    "something, getting reindeers",
                    "and elves to work for him.",
                    "The man must be a genius!"
                ],
            )?;
            return ctx.close();
        }
        _ => {
            ctx.lines_as(
                "Pilot",
                args![
                    "You know, this whole",
                    "piloting thing in the air,",
                    "it's rather new, you know?",
                    "Yeah, they got this Airship",
                    "operation in a hurry."
                ],
            )?;
            ctx.next()?;
            ctx.npc().emotion(constants::ET_HUK)?;
            ctx.lines_as(
                "Pilot",
                args![
                    "Still, they were real",
                    "serious, really thought",
                    "ahead. I mean, they had us",
                    "training while the Airships",
                    "were still being invented.",
                    "Isn't that freakin' crazy?!"
                ],
            )?;
            return ctx.close();
        }
    }
}

pub fn apple_merchant_airplane(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    let mut l_pay = Val::from(0);
    ctx.lines_as(
        "Fruitz",
        args![
            "Welcome to Fruitz's",
            "Shop where you can",
            "purchase Apples or grind",
            "them to make Apple Juice."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Buy Apples.", "Make Apple Juice.", "Why are you here?", "Cancel."])? {
        0 => {
            ctx.lines_as(
                "Fruitz",
                args![
                    "Please enter the amount",
                    "of Apples that you wish to",
                    "buy. Each Apple is 15 zeny",
                    "and you can buy a maximum",
                    "of 500 at a time. Please enter",
                    " '0' to cancel your order."
                ],
            )?;
            ctx.next()?;
            loop {
                let (input, _) = runtime::input_number(ctx, None, None)?;
                l_input = input;
                l_pay = Val::from(l_input.number()? * 15);
                if l_input == 0 {
                    ctx.lines_as(
                        "Fruitz",
                        args![
                            "Thanks for stopping",
                            "by my shop. Farewell!",
                            "Come by anytime when",
                            "you feel like having an",
                            "Apple to snack on~"
                        ],
                    )?;
                    return ctx.close();
                } else if l_input.number()? < 1 || l_input.number()? > 500 {
                    ctx.lines_as(
                        "Fruitz",
                        args![
                            "You've entered a number",
                            "higher than the maximum",
                            "value of 500. Please enter",
                            "the number of Apples you",
                            "wish to purchase again."
                        ],
                    )?;
                    ctx.next()?;
                } else {
                    ctx.lines_as(
                        "Fruitz",
                        args![
                            ((Val::from("A total of ^FF0000") + l_input.clone()) + Val::from("^000000 Apples")),
                            ((Val::from("will cost you ^FF0000") + l_pay.clone()) + Val::from("^000000 zeny.")),
                            "Would you like to continue?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Yes", "No"])? == 1 {
                        ctx.lines_as(
                            "Fruitz",
                            args![
                                "Thanks for stopping",
                                "by my shop. Farewell!",
                                "Come by anytime when",
                                "you feel like having an",
                                "Apple to snack on~"
                            ],
                        )?;
                        return ctx.close();
                    }
                    break;
                }
            }
            if ctx.player().zeny()? < l_pay.number()? {
                ctx.lines_as(
                    "Fruitz",
                    args![
                        "I'm sorry, but you don't",
                        "have enough money to",
                        "purchase that many Apples.",
                        "Please check your zeny or",
                        "purchase fewer Apples."
                    ],
                )?;
                return ctx.close();
            } else if ctx.call(Function::CheckWeight, args![512, l_input.clone()])? == 0 {
                ctx.lines_as(
                    "Fruitz",
                    args![
                        "Hmmm, I don't think",
                        "you've got enough room in",
                        "your inventory to carry this",
                        "many Apples. Why don't you free up some of your inventory space?"
                    ],
                )?;
                return ctx.close();
            } else {
                ctx.player().set_zeny(ctx.player().zeny()? - l_pay.number()?)?;
                ctx.call(Function::GetItem, args![512, l_input.clone()])?;
                ctx.lines_as(
                    "Fruitz",
                    args![
                        "Thanks for stopping by",
                        "my shop. I hope you enjoy",
                        "the flavor of these Apples~!"
                    ],
                )?;
                return ctx.close();
            }
        }
        1 => {
            ctx.lines_as(
                "Fruitz",
                args![
                    "Okay, I'll need",
                    "^FF00003 Apples and 1 Empty Bottle^000000",
                    "to make 1 Apple Juice for you.",
                    "Would you like to proceed?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    if ctx.items().count(512)? < 3 || ctx.items().count(713)? < 1 {
                        ctx.lines_as(
                            "Fruitz",
                            args![
                                "I'm sorry, but you don't",
                                "have enough materials to",
                                "create a bottle of Apple Juice.",
                                "Remember, I need 3 Apples",
                                "and 1 Empty Bottle to do it."
                            ],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.lines_as("Fruitz", args!["Thank you,", "please wait", "just a moment."])?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FF*Grind grind*", "*Grind grind*", "*Clang...!*^000000"])?;
                        ctx.next()?;
                        ctx.items().take(512, 3)?;
                        ctx.items().take(713, 1)?;
                        ctx.items().give(531, 1)?;
                        ctx.lines_as(
                            "Fruitz",
                            args![
                                "There you go~",
                                "I hope you enjoy!",
                                "Please feel free to",
                                "stop by for your Apple",
                                "and Apple Juice needs",
                                "at anytime, adventurer~"
                            ],
                        )?;
                        return ctx.close();
                    }
                }
                _ => {
                    ctx.lines_as(
                        "Fruitz",
                        args![
                            "Thanks for stopping",
                            "by my shop. Farewell!",
                            "Come by anytime when",
                            "you feel like having an",
                            "Apple to snack on~"
                        ],
                    )?;
                    return ctx.close();
                }
            }
        }
        2 => {
            ctx.lines_as(
                "Fruitz",
                args![
                    "I used to be a wandering",
                    "vagabond when, one day,",
                    "I took a nap and something",
                    "struck my head and awoke",
                    "me from my restful slumber."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fruitz",
                args![
                    "It turns out that I was",
                    "sleeping beneath an apple",
                    "tree and that an apple fell",
                    "and hit me on the head.",
                    "I was dying of hunger and",
                    "was about to eat that Apple..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fruitz",
                args![
                    "But suddenly, Kain, my old",
                    "friend from the mining days,",
                    "asked me to help him around",
                    "on the Airship. So I did, and",
                    "it was there where I found some",
                    "people playing the Dice game."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fruitz",
                args![
                    "I was bored and curious",
                    "and ended up wagering that",
                    "single Apple in a game of",
                    "dice. But for some reason,",
                    "I had this incredible lucky",
                    "streak. One apple became two... "
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fruitz",
                args![
                    "Two became four and",
                    "before I knew it, I had",
                    "cornered the Apple market!",
                    "I won so many Apples, I just",
                    "started my own business here",
                    "on the Airship. Weird, huh?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Fruitz",
                args![
                    "So Apples are good",
                    "for you. They were",
                    "certainly very good",
                    "to me. Hahahahaah~!"
                ],
            )?;
            return ctx.close();
        }
        _ => {
            ctx.lines_as("Fruitz", args!["Thank you for", "using my shop.", "Farewell~"])?;
            return ctx.close();
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Airshipwarp3Step {
    Start,
    OnTouch,
    OnInit,
    OnHide,
    OnUnhide,
}

fn airshipwarp_3_run(ctx: &Ctx, mut step: Airshipwarp3Step, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            Airshipwarp3Step::Start => {
                return Err(Stop::End);
            }
            Airshipwarp3Step::OnTouch => {
                let subject1 = ctx.var("$@airplanelocation2").get()?;
                if subject1 == 0 {
                    ctx.warp("ra_fild12", 292, 204)?;
                    return Err(Stop::End);
                } else if subject1 == 1 {
                    ctx.warp("izlude", 200, 56)?;
                    return Err(Stop::End);
                } else if subject1 == 2 {
                    ctx.warp("yuno", 12, 261)?;
                    return Err(Stop::End);
                }
                step = Airshipwarp3Step::OnInit;
                continue 'machine;
            }
            Airshipwarp3Step::OnInit => {
                step = Airshipwarp3Step::OnHide;
                continue 'machine;
            }
            Airshipwarp3Step::OnHide => {
                ctx.npc().special_effect(constants::EF_BASH)?;
                ctx.call(Function::DisableNpc, args![])?;
                return Err(Stop::End);
            }
            Airshipwarp3Step::OnUnhide => {
                ctx.call(Function::EnableNpc, args![])?;
                ctx.npc().special_effect(constants::EF_SUMMONSLAVE)?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn airshipwarp_3(ctx: &Ctx) -> Script {
    airshipwarp_3_run(ctx, Airshipwarp3Step::Start, Vec::new()).map(|_| ())
}

pub fn airshipwarp_3_ontouch(ctx: &Ctx) -> Script {
    airshipwarp_3_run(ctx, Airshipwarp3Step::OnTouch, Vec::new()).map(|_| ())
}

pub fn airshipwarp_3_oninit(ctx: &Ctx) -> Script {
    airshipwarp_3_run(ctx, Airshipwarp3Step::OnInit, Vec::new()).map(|_| ())
}

pub fn airshipwarp_3_onhide(ctx: &Ctx) -> Script {
    airshipwarp_3_run(ctx, Airshipwarp3Step::OnHide, Vec::new()).map(|_| ())
}

pub fn airshipwarp_3_onunhide(ctx: &Ctx) -> Script {
    airshipwarp_3_run(ctx, Airshipwarp3Step::OnUnhide, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum InternationalAirshipStep {
    Start,
    OnInit,
    OnEnable,
    OnTimer25000,
    OnTimer50000,
    OnTimer60000,
    OnTimer70000,
    OnTimer80000,
    OnTimer105000,
    OnTimer130000,
    OnTimer140000,
    OnTimer150000,
    OnTimer160000,
    OnTimer185000,
    OnTimer210000,
    OnTimer220000,
    OnTimer230000,
    OnTimer240000,
}

fn international_airship_run(ctx: &Ctx, mut step: InternationalAirshipStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            InternationalAirshipStep::Start => {
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnInit => {
                step = InternationalAirshipStep::OnEnable;
                continue 'machine;
            }
            InternationalAirshipStep::OnEnable => {
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer25000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "We are heading to Izlude.", constants::BC_MAP, "0x00ff00"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer50000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "We will arrive in Izlude shortly.", constants::BC_MAP, "0x00ff00"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer60000 => {
                ctx.var("$@airplanelocation2").set(Val::from(1))?;
                ctx.npc().do_event("#AirshipWarp-3::OnUnhide")?;
                ctx.npc().do_event("#AirshipWarp-4::OnUnhide")?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "Welcome to Izlude. Have a safe trip.", constants::BC_MAP, "0x00ff00"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer70000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "airplane_01",
                        "We are currently in Izlude. The Airship will take off shortly.",
                        constants::BC_MAP,
                        "0x00ff00"
                    ],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer80000 => {
                ctx.npc().do_event("#AirshipWarp-3::OnHide")?;
                ctx.npc().do_event("#AirshipWarp-4::OnHide")?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "airplane_01",
                        "The Airship is now taking off. Our next destination is Juno.",
                        constants::BC_MAP,
                        "0x70dbdb"
                    ],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer105000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "We are heading to Juno.", constants::BC_MAP, "0x70dbdb"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer130000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "We will arrive in Juno shortly.", constants::BC_MAP, "0x70dbdb"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer140000 => {
                ctx.var("$@airplanelocation2").set(Val::from(2))?;
                ctx.npc().do_event("#AirshipWarp-3::OnUnhide")?;
                ctx.npc().do_event("#AirshipWarp-4::OnUnhide")?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "Welcome to Juno. Have a safe trip.", constants::BC_MAP, "0x70dbdb"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer150000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "airplane_01",
                        "We are currently in Juno. The Airship will leave shortly.",
                        constants::BC_MAP,
                        "0x70dbdb"
                    ],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer160000 => {
                ctx.npc().do_event("#AirshipWarp-3::OnHide")?;
                ctx.npc().do_event("#AirshipWarp-4::OnHide")?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "airplane_01",
                        "The Airship is leaving the ground. Our next destination is Rachel.",
                        constants::BC_MAP,
                        "0xFF8200"
                    ],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer185000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "We are heading to Rachel.", constants::BC_MAP, "0xFF8200"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer210000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "We will arrive in Rachel shortly.", constants::BC_MAP, "0xFF8200"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer220000 => {
                ctx.var("$@airplanelocation2").set(Val::from(0))?;
                ctx.npc().do_event("#AirshipWarp-3::OnUnhide")?;
                ctx.npc().do_event("#AirshipWarp-4::OnUnhide")?;
                ctx.call(
                    Function::MapAnnounce,
                    args!["airplane_01", "Welcome to Rachel. Have a safe trip.", constants::BC_MAP, "0xFF8200"],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer230000 => {
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "airplane_01",
                        "We are currently in Rachel. The Airship will take off shortly.",
                        constants::BC_MAP,
                        "0xFF8200"
                    ],
                )?;
                return Err(Stop::End);
            }
            InternationalAirshipStep::OnTimer240000 => {
                ctx.npc().do_event("#AirshipWarp-3::OnHide")?;
                ctx.npc().do_event("#AirshipWarp-4::OnHide")?;
                ctx.call(
                    Function::MapAnnounce,
                    args![
                        "airplane_01",
                        "The Airship is now taking off. Our next destination is Izlude.",
                        constants::BC_MAP,
                        "0x00ff00"
                    ],
                )?;
                ctx.call(Function::StopNpcTimer, args![])?;
                ctx.var(".moninv").set(ctx.var(".moninv").get()?.number()? + 1)?;
                if ctx.var(".moninv").get()? == 7 {
                    if ctx.rand_range(1, 3)? == 3 {
                        ctx.npc().do_event("Airship#airplane02::OnEnable")?;
                        return Err(Stop::End);
                    }
                    ctx.var(".moninv").set(Val::from(0))?;
                }
                ctx.call(Function::InitNpcTimer, args![])?;
                return Err(Stop::End);
            }
        }
    }
}

pub fn international_airship(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::Start, Vec::new()).map(|_| ())
}

pub fn international_airship_oninit(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnInit, Vec::new()).map(|_| ())
}

pub fn international_airship_onenable(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnEnable, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer25000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer25000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer50000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer50000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer60000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer60000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer70000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer70000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer80000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer80000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer105000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer105000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer130000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer130000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer140000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer140000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer150000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer150000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer160000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer160000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer185000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer185000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer210000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer210000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer220000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer220000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer230000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer230000, Vec::new()).map(|_| ())
}

pub fn international_airship_ontimer240000(ctx: &Ctx) -> Script {
    international_airship_run(ctx, InternationalAirshipStep::OnTimer240000, Vec::new()).map(|_| ())
}

pub fn exit_airplane_011a(ctx: &Ctx) -> Script {
    ctx.end()
}

pub fn airship_staff_airplane01(ctx: &Ctx) -> Script {
    ctx.lines_as("Airship Staff", args!["Welcome", "to the Airship.", "How may I help you?"])?;
    ctx.next()?;
    match ctx.menu(&["Using the Airship", "Captain's Cabin", "Facilities", "Cancel"])? {
        0 => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "When you see a broadcast",
                    "announcing that we have",
                    "arrived at your destination,",
                    "please use one of the exits",
                    "located at the north and",
                    "south ends of the Airship."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Airship Staff",
                args![
                    "If you happen to miss",
                    "your stop, don't worry.",
                    "The Airship is constantly",
                    "en route and you'll get",
                    "another chance to arrive",
                    "to your intended destination."
                ],
            )?;
            return ctx.close();
        }
        1 => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "The Captain's Cabin",
                    "is located at the front",
                    "of the Airship. There, you",
                    "can meet the captain and",
                    "the pilot of the Airship."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "The Airship provides",
                    "various Mini Games for",
                    "the entertainment of all",
                    "our passengers. We invite",
                    "you to try your luck and skills",
                    "in the Airship's Mini Games~"
                ],
            )?;
            return ctx.close();
        }
        _ => {
            ctx.lines_as(
                "Airship Staff",
                args![
                    "Well, I hope you",
                    "your flight aboard",
                    "our Airships. Thank",
                    "you and have a good day."
                ],
            )?;
            return ctx.close();
        }
    }
}

pub fn apple_merchant_air01(ctx: &Ctx) -> Script {
    let mut l_input = Val::from(0);
    let mut l_pay = Val::from(0);
    if ctx.call(Function::CheckWeight, args![1201, 1])? == 0 {
        ctx.lines(args![
            "- Wait a minute !! -",
            "- Currently you're carrying -",
            "- too many items with you. -",
            "- Please try again -",
            "- after you lose some weight. -"
        ])?;
        return ctx.close();
    }
    ctx.lines_as(
        "Meltz",
        args![
            "Welcome to Meltz's",
            "Shop where you can",
            "purchase Apples or grind",
            "them to make Apple Juice."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Buy Apples.", "Make Apple Juice.", "Cancel."])? {
        0 => {
            ctx.lines_as(
                "Meltz",
                args![
                    "Please enter the amount",
                    "of Apples that you wish to",
                    "buy. Each Apple is 15 zeny",
                    "and you can buy a maximum",
                    "of 500 at a time. Please enter",
                    "'0' to cancel your order."
                ],
            )?;
            ctx.next()?;
            loop {
                let (input, _) = runtime::input_number(ctx, None, None)?;
                l_input = input;
                l_pay = Val::from(l_input.number()? * 15);
                if l_input == 0 {
                    ctx.lines_as(
                        "Meltz",
                        args![
                            "Thanks for stopping",
                            "by my shop. Farewell!",
                            "Come by anytime when",
                            "you feel like having an",
                            "Apple to snack on~"
                        ],
                    )?;
                    return ctx.close();
                } else if l_input.number()? < 1 || l_input.number()? > 500 {
                    ctx.lines_as(
                        "Meltz",
                        args![
                            "You've entered a number",
                            "higher than the maximum",
                            "value of 500. Please enter",
                            "the number of Apples you",
                            "wish to purchase again."
                        ],
                    )?;
                    ctx.next()?;
                } else {
                    ctx.lines_as(
                        "Meltz",
                        args![
                            ((Val::from("A total of ^FF0000") + l_input.clone()) + Val::from("^000000 Apples")),
                            ((Val::from("will cost you ^FF0000") + l_pay.clone()) + Val::from("^000000 zeny.")),
                            "Would you like to continue?"
                        ],
                    )?;
                    ctx.next()?;
                    if ctx.menu(&["Yes", "No"])? == 1 {
                        ctx.lines_as(
                            "Meltz",
                            args![
                                "Thanks for stopping",
                                "by my shop. Farewell!",
                                "Come by anytime when",
                                "you feel like having an",
                                "Apple to snack on~"
                            ],
                        )?;
                        return ctx.close();
                    }
                    break;
                }
            }
            if ctx.player().zeny()? < l_pay.number()? {
                ctx.lines_as(
                    "Meltz",
                    args![
                        "I'm sorry, you don't have",
                        "enough money with you.",
                        "Please check your funds or",
                        "purchase less Apples."
                    ],
                )?;
                return ctx.close();
            } else if ctx.call(Function::CheckWeight, args![512, l_input.clone()])? == 0 {
                ctx.lines_as(
                    "Meltz",
                    args![
                        "Hmm, I don't think you've",
                        "got enough room to carry",
                        "this many Apples. You might",
                        "want to free up your inventory",
                        "space."
                    ],
                )?;
                return ctx.close();
            } else {
                ctx.player().set_zeny(ctx.player().zeny()? - l_pay.number()?)?;
                ctx.call(Function::GetItem, args![512, l_input.clone()])?;
                ctx.lines_as(
                    "Meltz",
                    args![
                        "Thanks for stopping by",
                        "my shop. I hope you enjoy",
                        "the flavor of these Apples~!"
                    ],
                )?;
                return ctx.close();
            }
        }
        1 => {
            ctx.lines_as(
                "Meltz",
                args![
                    "Okay, I'll need",
                    "^FF00003 Apples and 1 Empty Bottle^000000",
                    "to make 1 Apple Juice for you.",
                    "Would you like to proceed?"
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Yes", "No"])? {
                0 => {
                    if ctx.items().count(512)? < 3 || ctx.items().count(713)? < 1 {
                        ctx.lines_as(
                            "Meltz",
                            args![
                                "I'm sorry, but you don't",
                                "have enough materials to",
                                "create a bottle of Apple Juice.",
                                "Remember, I need 3 Apples",
                                "and 1 Empty Bottle to do it."
                            ],
                        )?;
                        return ctx.close();
                    } else {
                        ctx.lines_as("Meltz", args!["Thank you, please wait."])?;
                        ctx.next()?;
                        ctx.lines(args!["^3355FF*Grind* *Grind*", "*Grind* *Grind*", "*Clang...!*^000000"])?;
                        ctx.next()?;
                        ctx.items().take(512, 3)?;
                        ctx.items().take(713, 1)?;
                        ctx.items().give(531, 1)?;
                        ctx.lines_as("Meltz", args!["There you go~", "Please come again."])?;
                        return ctx.close();
                    }
                }
                _ => {
                    ctx.lines_as(
                        "Meltz",
                        args![
                            "Thanks for stopping",
                            "by my shop. Farewell!",
                            "Come by anytime when",
                            "you feel like having an",
                            "Apple to snack on~"
                        ],
                    )?;
                    return ctx.close();
                }
            }
        }
        _ => {
            ctx.lines_as(
                "Meltz",
                args![
                    "Thanks for stopping",
                    "by my shop. Farewell!",
                    "Come by anytime when",
                    "you feel like having an",
                    "Apple to snack on~"
                ],
            )?;
            return ctx.close();
        }
    }
}

pub fn pilot_airplane_01(ctx: &Ctx) -> Script {
    match ctx.rand_range(1, 4)? {
        1 => {
            ctx.lines_as(
                "Pilot",
                args![
                    "Longitude, 131 degrees east.",
                    "Latitude, 37 degrees north.",
                    "We're right on course, captain."
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Pilot",
                args![
                    "Looks like a really",
                    "cloudy day. Always hard",
                    "to navigate when the skies",
                    "aren't clear. Guess we'll",
                    "need to amp the radar."
                ],
            )?;
            return ctx.close();
        }
        3 => {
            ctx.lines_as(
                "Pilot",
                args![
                    "The Captain is a good",
                    "man and I can't think of",
                    "a finer person to command",
                    "this ship. Still, he's pretty",
                    "tough, a real slave driver."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "^ff0000Tarlock^000000",
                args![
                    "^ff0000Hey...!^000000",
                    "^ff0000Less chit-chat^000000",
                    "^ff0000and more piloting!^000000"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Pilot", args!["R-right away, sir!", "(See what I mean?)"])?;
            return ctx.close();
        }
        _ => {
            ctx.lines_as(
                "Pilot",
                args![
                    "This uniform is",
                    "really dapper, but",
                    "it's way too thick to",
                    "wear around the Airship."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Pilot",
                args![
                    "...",
                    "......",
                    "No one ever really",
                    "comes into this room.",
                    "And the captain IS a reindeer.",
                    "I could just strip to my boxers."
                ],
            )?;
            ctx.next()?;
            ctx.npc().emotion(constants::ET_HUK)?;
            ctx.lines_as("Pilot", args!["Oh...! Hello there!", "E-e-enjoying your flight?!"])?;
            return ctx.close();
        }
    }
}

pub fn dianne_01airplane_01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Dianne",
        args![
            "It's so weird!",
            "I went to visit the",
            "Airship Captain and",
            "all I saw was this",
            "weird reindeer. Oh!",
            "Do you think that..."
        ],
    )?;
    ctx.close()
}

pub fn dianne_01airplane_01_ontouch(ctx: &Ctx) -> Script {
    ctx.npc().emotion(constants::ET_CRY)?;
    ctx.end()
}

pub fn mendel_01airplane_01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Mendel",
        args![
            "As I expected, the",
            "in-flight meals are",
            "three star quality at best.",
            "*Harrrumph* I really should",
            "have brought my chef so that",
            "I could enjoy a real meal."
        ],
    )?;
    ctx.close()
}

pub fn swordsman_shimizu_air_01(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Swordsman Shimizu",
        args!["Finally, after five", "years of waiting...", "I can have my revenge!"],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Swordsman Shimizu",
        args![
            "I just...",
            "Have to make sure that",
            "I don't keep missing my",
            "stop. But soon, very soon,",
            "vengeance will be mine!"
        ],
    )?;
    ctx.close()
}

pub fn nils_ein(ctx: &Ctx) -> Script {
    let mut l_end_time = Val::from(0);
    let mut l_letters: Vec<Val> = Vec::new();
    let mut l_line1_1_s: Vec<Val> = Vec::new();
    let mut l_line1_2_s: Vec<Val> = Vec::new();
    let mut l_line1_3_s: Vec<Val> = Vec::new();
    let mut l_line2_1_s: Vec<Val> = Vec::new();
    let mut l_line2_2_s: Vec<Val> = Vec::new();
    let mut l_save1_s = Val::from("");
    let mut l_save2_s = Val::from("");
    let mut l_start_time = Val::from(0);
    let mut l_tasoo = Val::from(0);
    let mut l_total_time = Val::from(0);
    let mut l_word1_s: Vec<Val> = Vec::new();
    let mut l_word2_s: Vec<Val> = Vec::new();
    let mut l_wordtest = Val::from(0);
    ctx.lines_as(
        "Nils",
        args![
            "Welcome to the",
            "^ff0000RO Typing Challenge^000000.",
            "Would you like to play",
            "a quick typing game?"
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&[
        "Play ^ff0000RO Typing Challenge^000000",
        "Information",
        "View Top Records",
        "Cancel",
    ])? {
        0 => {
            ctx.lines_as(
                "Nils",
                args![
                    "Okay, we have",
                    "a new challenger!",
                    "Enter the following",
                    "text as quickly as you",
                    "can without making any",
                    "mistakes! Let's start~!"
                ],
            )?;
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_line1_1_s,
                &Val::from(base + 0),
                Val::from("^3cbcbccallipygian salacius lascivious^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_1_s,
                &Val::from(base + 1),
                Val::from("^3cbcbcBy the power of^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_1_s,
                &Val::from(base + 2),
                Val::from("^0000ffthkelfkskeldmsiejdlslehfndkelsheidl^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_1_s,
                &Val::from(base + 3),
                Val::from("^3cbcbcburrdingdingdingdilidingdingdingphoohudaamb^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_1_s,
                &Val::from(base + 4),
                Val::from("^3cbcbcCoboman no chikara-yumei na^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_1_s,
                &Val::from(base + 5),
                Val::from("^3cbcbcI'm the king of All Weirdos! Now^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_1_s,
                &Val::from(base + 6),
                Val::from("^3cbcbcYou give me no choice. I guess it's^000000"),
                true,
            );
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_line1_2_s,
                &Val::from(base + 0),
                Val::from("^3cbcbclicentious prurient concupiscent^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_2_s,
                &Val::from(base + 1),
                Val::from("^3cbcbcp-po-poi-po-poi-poin-poing^000000"),
                true,
            );
            runtime::local_set(&mut l_line1_2_s, &Val::from(base + 2), Val::from("^3cbcbcskemd^000000"), true);
            runtime::local_set(
                &mut l_line1_2_s,
                &Val::from(base + 3),
                Val::from("^3cbcbcandoorabambarambambambambamburanbamding^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_2_s,
                &Val::from(base + 4),
                Val::from("^3cbcbcchikara-daiookii na chikara da ze!^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_2_s,
                &Val::from(base + 5),
                Val::from("^3cbcbcyou know of my true power. Obey~!^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line1_2_s,
                &Val::from(base + 6),
                Val::from("^3cbcbctime for me to reveal my secret...^000000"),
                true,
            );
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_line1_3_s, &Val::from(base + 0), Val::from(""), true);
            runtime::local_set(
                &mut l_line1_3_s,
                &Val::from(base + 1),
                Val::from("^3cbcbcGOD-POING. I NEVER LOSE!^000000"),
                true,
            );
            runtime::local_set(&mut l_line1_3_s, &Val::from(base + 2), Val::from(""), true);
            runtime::local_set(&mut l_line1_3_s, &Val::from(base + 3), Val::from(""), true);
            runtime::local_set(&mut l_line1_3_s, &Val::from(base + 4), Val::from("^3cbcbcCOBO ON^000000"), true);
            runtime::local_set(&mut l_line1_3_s, &Val::from(base + 5), Val::from(""), true);
            runtime::local_set(&mut l_line1_3_s, &Val::from(base + 6), Val::from(""), true);
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_word1_s,
                &Val::from(base + 0),
                Val::from("callipygian salacius lascivious licentious prurient concupiscent"),
                true,
            );
            runtime::local_set(
                &mut l_word1_s,
                &Val::from(base + 1),
                Val::from("By the power of p-po-poi-po-poi-poin-poing GOD-POING. I NEVER LOSE!"),
                true,
            );
            runtime::local_set(
                &mut l_word1_s,
                &Val::from(base + 2),
                Val::from("thkelfkskeldmsiejdlslehfndkelsheidlskemd"),
                true,
            );
            runtime::local_set(
                &mut l_word1_s,
                &Val::from(base + 3),
                Val::from("burrdingdingdingdilidingdingdingphoohudaambandoorabambarambambambambamburanbamding"),
                true,
            );
            runtime::local_set(
                &mut l_word1_s,
                &Val::from(base + 4),
                Val::from("Coboman no chikara-yumei na chikara-daiookii na chikara da ze! COBO ON"),
                true,
            );
            runtime::local_set(
                &mut l_word1_s,
                &Val::from(base + 5),
                Val::from("I'm the king of All Weirdos! Now you know of my true power. Obey~!"),
                true,
            );
            runtime::local_set(
                &mut l_word1_s,
                &Val::from(base + 6),
                Val::from("You give me no choice. I guess it's time for me to reveal my secret..."),
                true,
            );
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_line2_1_s,
                &Val::from(base + 0),
                Val::from("^3cbcbcuNflAPPaBLe LoVaBLe SeCreTs AnD^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_1_s,
                &Val::from(base + 1),
                Val::from("^ff1493LiGhTsPeEd RiGhT SPEed LeFT TURn^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_1_s,
                &Val::from(base + 2),
                Val::from("^ff1493hfjdkeldjsieldjshfjdjeiskdlefvbd^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_1_s,
                &Val::from(base + 3),
                Val::from("^ff1493burapaphuralanderamduanbatuhiwooi^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_1_s,
                &Val::from(base + 4),
                Val::from("^ff1493belief love luck grimace sweat rush^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_1_s,
                &Val::from(base + 5),
                Val::from("^800080opeN, Open!op3n.openOpen0p3nOpEn0pen^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_1_s,
                &Val::from(base + 6),
                Val::from("^3cbcbcfReeDoM ecstAcy JoUrnaliSm ArMplt^000000"),
                true,
            );
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_line2_2_s,
                &Val::from(base + 0),
                Val::from("^3cbcbcboWLIiNg aGaINST tHe KarMA of YoUtH^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_2_s,
                &Val::from(base + 1),
                Val::from("^ff1493RiGhT BuRn OrIGInAL GaNgSteR SmACk^000000"),
                true,
            );
            runtime::local_set(&mut l_line2_2_s, &Val::from(base + 2), Val::from(""), true);
            runtime::local_set(
                &mut l_line2_2_s,
                &Val::from(base + 3),
                Val::from("^ff1493kabamturubamdingding^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_2_s,
                &Val::from(base + 4),
                Val::from("^ff1493folktale rodimus optimus bumblebee^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_2_s,
                &Val::from(base + 5),
                Val::from("^800080`open'0Pen open? open!111OPENSESAME^000000"),
                true,
            );
            runtime::local_set(
                &mut l_line2_2_s,
                &Val::from(base + 6),
                Val::from("^3cbcbcDisCoverY hEaDaChE MoonbeAmS jUsTiCE^000000"),
                true,
            );
            let base = Val::from(0).number()?;
            runtime::local_set(
                &mut l_word2_s,
                &Val::from(base + 0),
                Val::from("uNflAPPaBLe LoVaBLe SeCreTs AnD boWLIiNg aGaINST tHe KarMA of YoUtH"),
                true,
            );
            runtime::local_set(
                &mut l_word2_s,
                &Val::from(base + 1),
                Val::from("LiGhTsPeEd RiGhT SPEed LeFT TURn RiGhT BuRn OrIGInAL GaNgSteR SmACk"),
                true,
            );
            runtime::local_set(
                &mut l_word2_s,
                &Val::from(base + 2),
                Val::from("hfjdkeldjsieldjshfjdjeiskdlefvbd"),
                true,
            );
            runtime::local_set(
                &mut l_word2_s,
                &Val::from(base + 3),
                Val::from("burapaphuralanderamduanbatuhiwooikabamturubamdingding"),
                true,
            );
            runtime::local_set(
                &mut l_word2_s,
                &Val::from(base + 4),
                Val::from("belief love luck grimace sweat rush folktale rodimus optimus bumblebee"),
                true,
            );
            runtime::local_set(
                &mut l_word2_s,
                &Val::from(base + 5),
                Val::from("opeN, Open!op3n.openOpen0p3nOpEn0pen`open'0Pen open? open!111OPENSESAME"),
                true,
            );
            runtime::local_set(
                &mut l_word2_s,
                &Val::from(base + 6),
                Val::from("fReeDoM ecstAcy JoUrnaliSm ArMplt DisCoverY hEaDaChE MoonbeAmS jUsTiCE"),
                true,
            );
            let base = Val::from(0).number()?;
            runtime::local_set(&mut l_letters, &Val::from(base + 0), Val::from(1300), false);
            runtime::local_set(&mut l_letters, &Val::from(base + 1), Val::from(1250), false);
            runtime::local_set(&mut l_letters, &Val::from(base + 2), Val::from(1180), false);
            runtime::local_set(&mut l_letters, &Val::from(base + 3), Val::from(1380), false);
            runtime::local_set(&mut l_letters, &Val::from(base + 4), Val::from(1740), false);
            runtime::local_set(&mut l_letters, &Val::from(base + 5), Val::from(1440), false);
            runtime::local_set(&mut l_letters, &Val::from(base + 6), Val::from(1450), false);
            l_wordtest = ctx.call(Function::Rand, args![7])?;
            ctx.next()?;
            ctx.lines_as(
                "Nils",
                args![
                    runtime::local_get(&l_line1_1_s, &l_wordtest, true),
                    runtime::local_get(&l_line1_2_s, &l_wordtest, true),
                    runtime::local_get(&l_line1_3_s, &l_wordtest, true)
                ],
            )?;
            l_start_time = ctx.call(Function::GetTimeTick, args![1])?;
            ctx.next()?;
            let (input, _) = runtime::input_text(ctx, None, None)?;
            l_save1_s = input;
            l_end_time = ctx.call(Function::GetTimeTick, args![1])?;
            l_total_time = (l_end_time.clone().try_sub(l_start_time.clone())?);
            ctx.lines_as(
                "Nils",
                args![
                    runtime::local_get(&l_line2_1_s, &l_wordtest, true),
                    runtime::local_get(&l_line2_2_s, &l_wordtest, true)
                ],
            )?;
            l_start_time = ctx.call(Function::GetTimeTick, args![1])?;
            ctx.next()?;
            let (input, _) = runtime::input_text(ctx, None, None)?;
            l_save2_s = input;
            l_end_time = ctx.call(Function::GetTimeTick, args![1])?;
            l_total_time = (l_total_time.clone() + (l_start_time.clone().try_sub(l_end_time.clone())?));
            l_tasoo = ((runtime::local_get(&l_letters, &l_wordtest, false).try_div(
                (if l_total_time.clone().number()? > 0 {
                    l_total_time.clone()
                } else {
                    Val::from(1)
                }),
            )?)
            .try_mul(Val::from(6))?);
            if l_save1_s.loosely_equals(&runtime::local_get(&l_word1_s, &l_wordtest, true))
                && l_save2_s.loosely_equals(&runtime::local_get(&l_word2_s, &l_wordtest, true))
            {
                ctx.lines_as(
                    "Nils",
                    args![
                        ((Val::from("Your record is ^ff0000") + l_total_time.clone()) + Val::from(" seconds^000000 and")),
                        ((Val::from("the total letters are ") + l_tasoo.clone()) + Val::from("."))
                    ],
                )?;
                ctx.next()?;
                if l_tasoo.clone().number()? >= 1300 {
                    ctx.lines_as(
                        "Nils",
                        args![
                            "Hmmm, this record isn't",
                            "humanly possible unless you",
                            "copy and paste the whole",
                            "sentence. Please play fairly",
                            "next time."
                        ],
                    )?;
                    return ctx.close();
                }
                if runtime::op(&l_tasoo.clone(), ">=", &ctx.var("$050320_ein_typing").get()?)?.is_true() {
                    ctx.lines_as(
                        "Nils",
                        args![
                            "The previous top record was",
                            ((Val::from("made by ^0000ff") + ctx.var("$050320_minus1_typing$").get()?) + Val::from("^000000")),
                            ((Val::from("with the total ^0000ff") + ctx.var("$050320_ein_typing").get()?) + Val::from("^000000 letters.")),
                            ((Val::from("However, ^ff0000") + ctx.player().name()?) + Val::from("^000000,")),
                            "you made the new top record",
                            "this time. Congratulations!"
                        ],
                    )?;
                    ctx.var("$050320_minus1_typing$").set(ctx.player().name()?)?;
                    ctx.var("$050320_ein_typing").set(l_tasoo.clone())?;
                    return ctx.close();
                } else {
                    ctx.lines_as(
                        "Nils",
                        args![
                            ((Val::from("^0000ff") + ctx.var("$050320_minus1_typing$").get()?) + Val::from("^000000")),
                            "is the current",
                            "record holder with",
                            ((Val::from("a letter total of ^0000ff") + ctx.var("$050320_ein_typing").get()?) + Val::from("^000000")),
                            "characters. Try to beat",
                            "that record next time~"
                        ],
                    )?;
                    return ctx.close();
                }
            } else {
                ctx.lines_as(
                    "Nils",
                    args!["Oooh...", "I'm sorry, but", "you entered the", "text incorrectly..."],
                )?;
                return ctx.close();
            }
        }
        1 => {
            ctx.lines_as(
                "Nils",
                args![
                    "The ^ff0000RO Typing Challenge^000000",
                    "is a game where you enter",
                    "the given text as quickly as you",
                    "can. The name of the top player",
                    "is recorded for posterity. If you",
                    "want fame, here's your chance!"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Nils",
                args![
                    "I'd just like to let",
                    "you know that you type",
                    "all the text that you see",
                    "in the single input line that",
                    "you're given. So don't press",
                    "the enter key, just click 'OK.'"
                ],
            )?;
            return ctx.close();
        }
        2 => {
            ctx.lines_as(
                "Nils",
                args![
                    ((Val::from("^0000ff") + ctx.var("$050320_minus1_typing$").get()?) + Val::from("^000000")),
                    "is the current",
                    "record holder with",
                    ((Val::from("a letter total of ^0000ff") + ctx.var("$050320_ein_typing").get()?) + Val::from("^000000")),
                    "characters. Try to beat",
                    "that record next time~"
                ],
            )?;
            return ctx.close();
        }
        _ => {
            ctx.lines_as(
                "Nils",
                args![
                    "Feel free to take on the",
                    "^ff0000RO Typing Challenge^000000",
                    "anytime. I'll be here~"
                ],
            )?;
            return ctx.close();
        }
    }
}

pub fn clarice(ctx: &Ctx) -> Script {
    ctx.lines_as(
        "Clarice",
        args![
            "Hi, I'm Clarice~",
            "How would you like",
            "to wager some Apples",
            "in a friendly game of Dice?"
        ],
    )?;
    ctx.next()?;
    shared::airports_airships::applegamble(ctx, args!["Clarice"])?;
    ctx.end()
}
