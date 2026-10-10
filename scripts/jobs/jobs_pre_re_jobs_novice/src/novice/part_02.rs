use script_sdk_2::{Ctx, Function, Script, Stop, Val, args, runtime};

fn kafra_employee_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_dest_s = Val::from("");
    let mut l_mapn_s = Val::from("");
    let mut l_savex = Val::from(0);
    let mut l_savey = Val::from(0);
    let mut l_warpx = Val::from(0);
    let mut l_warpy = Val::from(0);
    ctx.lines_as(
        "Kafra Employee",
        args!["Welcome to", "Kafra Corporation.", "The Kafra services are", "always on your side."],
    )?;
    if ctx.var("new_mes_flag0").get()?.is_true() {
        ctx.var("new_mes_flag0").set(Val::from(0))?;
        ctx.var("new_mes_flag1").set(Val::from(0))?;
        ctx.var("new_mes_flag2").set(Val::from(0))?;
        ctx.var("new_mes_flag3").set(Val::from(0))?;
        ctx.var("new_mes_flag4").set(Val::from(0))?;
        ctx.var("new_mes_flag5").set(Val::from(0))?;
        ctx.var("new_lvup0").set(Val::from(0))?;
        ctx.var("new_lvup1").set(Val::from(0))?;
        ctx.var("new_joblvup").set(Val::from(0))?;
    }
    ctx.next()?;
    ctx.lines_as(
        "Kafra Employee",
        args!["I've been dispatched from Kafra Corporation Headquarters to assist new players such as yourself."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Kafra Employee",
        args![
            "Please, take heed!",
            "If you move to a town",
            "^4d4dffyou will be unable to return to the Training Grounds ever again^000000."
        ],
    )?;
    ctx.next()?;
    if Val::from(runtime::select_values(
        ctx,
        &[Val::from("Teleport Service"), Val::from("About Kafra services")],
    )?) == 1
    {
        if ((ctx.var("nov_get_item02").get()?.number()? < 10 && ctx.var("nov_get_item03").get()?.number()? < 10)
            && ctx.var("nov_get_item04").get()?.number()? < 10)
        {
            ctx.lines_as("Kafra Employee", args!["I see, you must want to teleport to a town in Rune-Midgarts imediately. First, let me briefly inform you about the different towns and cities in Ragnarok."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Employee",
                args!["Prontera is the capital of the Rune-Midgarts kingdom, and its satellite, Izlude, is closeby."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "^996633Morocc^000000 is in the desert. It's the town where you can change your job to the Thief and Assassin classes."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Kafra Employee", args!["^006600Payon^000000 is in the mountains, and is famous for its Archer Village, where Novices can change their jobs to Archers."])?;
            ctx.next()?;
            ctx.lines_as(
                "Kafra Employee",
                args!["The city of magic, ^993300Geffen^000000, is where people go to become Mages and Wizards."],
            )?;
            ctx.next()?;
            ctx.lines_as("Kafra Employee", args!["^003399Alberta^000000, the port city, is where the Merchant Guild is located. You must also go to Alberta if you wish to travel by sea."])?;
            ctx.next()?;
            ctx.lines_as("Kafra Employee", args!["Please choose", "your destination."])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Prontera:Morocc:Payon:Alberta:Geffen")])? {
                1 => {
                    l_dest_s = Val::from("Prontera");
                    l_mapn_s = Val::from("prontera");
                    l_savex = Val::from(117);
                    l_savey = Val::from(72);
                    l_warpx = Val::from(150);
                    l_warpy = Val::from(50);
                }
                2 => {
                    l_dest_s = Val::from("Morocc");
                    l_mapn_s = Val::from("morocc");
                    l_savex = Val::from(150);
                    l_savey = Val::from(99);
                    l_warpx = Val::from(155);
                    l_warpy = Val::from(110);
                }
                3 => {
                    l_dest_s = Val::from("Payon");
                    l_mapn_s = Val::from("payon");
                    l_savex = Val::from(70);
                    l_savey = Val::from(100);
                    l_warpx = Val::from(166);
                    l_warpy = Val::from(67);
                }
                4 => {
                    l_dest_s = Val::from("Alberta");
                    l_mapn_s = Val::from("alberta");
                    l_savex = Val::from(30);
                    l_savey = Val::from(232);
                    l_warpx = Val::from(114);
                    l_warpy = Val::from(58);
                }
                5 => {
                    l_dest_s = Val::from("Geffen");
                    l_mapn_s = Val::from("geffen");
                    l_savex = Val::from(119);
                    l_savey = Val::from(37);
                    l_warpx = Val::from(122);
                    l_warpy = Val::from(65);
                }
                _ => {}
            }
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "You have decided",
                    ((Val::from("to go to ") + l_dest_s.clone()) + Val::from(".")),
                    "May God be with you."
                ],
            )?;
            ctx.close_window()?;
            if ctx.var("nov_get_item05").get()?.number()? < 11 {
                ctx.var("nov_get_item05").set(Val::from(11))?;
                ctx.call(Function::GetItem, vec![Val::from(569), Val::from(100)])?;
                ctx.call(Function::GetItem, vec![Val::from(1243), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(2414), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(2510), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(2352), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(2112), Val::from(1)])?;
                ctx.call(Function::GetItem, vec![Val::from(601), Val::from(10)])?;
                ctx.call(Function::GetItem, vec![Val::from(602), Val::from(2)])?;
                ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
            }
            ctx.var("nov_1st_cos").set(Val::from(0))?;
            ctx.var("nov_2nd_cos").set(Val::from(0))?;
            ctx.var("nov_3_swordman").set(Val::from(0))?;
            ctx.var("nov_3_archer").set(Val::from(0))?;
            ctx.var("nov_3_thief").set(Val::from(0))?;
            ctx.var("nov_3_magician").set(Val::from(0))?;
            ctx.var("nov_3_acolyte").set(Val::from(0))?;
            ctx.var("nov_3_merchant").set(Val::from(0))?;
            ctx.call(
                Function::SavePoint,
                vec![l_mapn_s.clone(), l_savex.clone(), l_savey.clone(), Val::from(1), Val::from(1)],
            )?;
            ctx.call(Function::Warp, vec![l_mapn_s.clone(), l_warpx.clone(), l_warpy.clone()])?;
            return Err(Stop::End);
        } else {
            match runtime::select_values(ctx, &[Val::from("Field Combat Course:Prontera:Morocc:Payon:Alberta:Geffen")])? {
                1 => {
                    ctx.lines_as(
                        "Kafra Employee",
                        args!["Thank you, let", "me send you to the", "Field Combat Training Course."],
                    )?;
                    ctx.close_window()?;
                    ctx.call(Function::Warp, vec![Val::from("new_1-2"), Val::from(28), Val::from(178)])?;
                    return Err(Stop::End);
                }
                2 => {
                    l_dest_s = Val::from("Prontera");
                    l_mapn_s = Val::from("prontera");
                    l_savex = Val::from(117);
                    l_savey = Val::from(72);
                    l_warpx = Val::from(150);
                    l_warpy = Val::from(50);
                }
                3 => {
                    l_dest_s = Val::from("Morocc");
                    l_mapn_s = Val::from("morocc");
                    l_savex = Val::from(150);
                    l_savey = Val::from(99);
                    l_warpx = Val::from(155);
                    l_warpy = Val::from(110);
                }
                4 => {
                    l_dest_s = Val::from("Payon");
                    l_mapn_s = Val::from("payon");
                    l_savex = Val::from(70);
                    l_savey = Val::from(100);
                    l_warpx = Val::from(166);
                    l_warpy = Val::from(67);
                }
                5 => {
                    l_dest_s = Val::from("Alberta");
                    l_mapn_s = Val::from("alberta");
                    l_savex = Val::from(30);
                    l_savey = Val::from(232);
                    l_warpx = Val::from(114);
                    l_warpy = Val::from(58);
                }
                6 => {
                    l_dest_s = Val::from("Geffen");
                    l_mapn_s = Val::from("geffen");
                    l_savex = Val::from(119);
                    l_savey = Val::from(37);
                    l_warpx = Val::from(122);
                    l_warpy = Val::from(65);
                }
                _ => {}
            }
            ctx.lines_as(
                "Kafra Employee",
                args![
                    "You have decided",
                    ((Val::from("to go to ") + l_dest_s.clone()) + Val::from(".")),
                    "May God be with you."
                ],
            )?;
            ctx.close_window()?;
            if ctx.var("nov_get_item05").get()?.number()? < 11 {
                ctx.var("nov_get_item05").set(Val::from(11))?;
                ctx.call(Function::GetItem, vec![Val::from(7059), Val::from(5)])?;
                ctx.call(Function::GetItem, vec![Val::from(7060), Val::from(5)])?;
            }
            ctx.var("nov_1st_cos").set(Val::from(0))?;
            ctx.var("nov_2nd_cos").set(Val::from(0))?;
            ctx.var("nov_3_swordman").set(Val::from(0))?;
            ctx.var("nov_3_archer").set(Val::from(0))?;
            ctx.var("nov_3_thief").set(Val::from(0))?;
            ctx.var("nov_3_magician").set(Val::from(0))?;
            ctx.var("nov_3_acolyte").set(Val::from(0))?;
            ctx.var("nov_3_merchant").set(Val::from(0))?;
            ctx.call(
                Function::SavePoint,
                vec![l_mapn_s.clone(), l_savex.clone(), l_savey.clone(), Val::from(1), Val::from(1)],
            )?;
            ctx.call(Function::Warp, vec![l_mapn_s.clone(), l_warpx.clone(), l_warpy.clone()])?;
            return Err(Stop::End);
        }
    } else {
        ctx.lines_as(
            "Kafra Employee",
            args![
                "Let me introduce you",
                "to the Kafra Services.",
                "In the menu, please choose",
                "the service you'd like to",
                "learn more about."
            ],
        )?;
        ctx.next()?;
        'l3: loop {
            if !(true) {
                break 'l3;
            }
            'b3: {
                match runtime::select_values(
                    ctx,
                    &[Val::from(
                        "Save service:Storage service:Teleport service:Cart rental service:Cancel",
                    )],
                )? {
                    1 => {
                        ctx.lines_as("Kafra Employee", args!["When you talk to a Kafra Employee and ask for the Save Service, the location of where you will revive, after being defeated in battle, will be changed."])?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["Your Respawn Point is always the last place where you have saved. Using a Butterfly Wing will return you to the place where you", "last saved."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kafra Employoee",
                            args![
                                "The Save Service",
                                "is also provided by",
                                "the Kafra Corporation",
                                "free of charge~!"
                            ],
                        )?;
                        if ctx.var("nov_1st_cos").get()?.number()? < 20 {
                            ctx.var("nov_1st_cos").set(Val::from(20))?;
                            let subject5 = ctx.var("BaseLevel").get()?;
                            if subject5 == 1 {
                                ctx.call(Function::GetExperience, vec![Val::from(10), Val::from(0)])?;
                            } else if subject5 == 2 {
                                ctx.call(Function::GetExperience, vec![Val::from(17), Val::from(0)])?;
                            } else if subject5 == 3 {
                                ctx.call(Function::GetExperience, vec![Val::from(26), Val::from(0)])?;
                            } else if subject5 == 4 {
                                ctx.call(Function::GetExperience, vec![Val::from(37), Val::from(0)])?;
                            } else if subject5 == 5 {
                                ctx.call(Function::GetExperience, vec![Val::from(78), Val::from(0)])?;
                            } else if subject5 == 6 {
                                ctx.call(Function::GetExperience, vec![Val::from(115), Val::from(0)])?;
                            } else if subject5 == 7 {
                                ctx.call(Function::GetExperience, vec![Val::from(155), Val::from(0)])?;
                            }
                        }
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Kafra Employee", args!["The Kafra Corporation is the world's largest company with a long and distinguished history on the Midgard continent."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "You can store and retrieve",
                                "your items in any town at your convenience. This Storage is shared by every character on one account."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["It's unreasonable to carry all of your items with you when you don't need them right away. Please use our Storage and keep your items safe and secure."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "Our convenient Storage Service",
                                "is provided to our customers for a small fee which is different from town to town."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "However, you must be",
                                "at least ^3355FFBasic Skill Level 6^000000",
                                "to use the Storage."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["There are 3 different item sections of the Storage into which items are organized: Consumable, Equipment and Etc."])?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["There are a maximum of 300 Inventory Slots in Kafra Storage, meaning you can have up to 300 different kinds of items in Storage."])?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["Remember though, that in the case of Equipment, each item takes up one Inventory Slot. The maximum number of items that can be placed in Kafra Storage is 30,000."])?;
                        if ctx.var("nov_3_archer").get()?.number()? < 20 && ctx.var("JobLevel").get()?.number()? < 7 {
                            ctx.var("nov_3_archer").set(Val::from(20))?;
                            let subject6 = ctx.var("JobLevel").get()?;
                            if subject6 == 1 {
                                ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(10)])?;
                            } else if subject6 == 2 {
                                ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(18)])?;
                            } else if subject6 == 3 {
                                ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(28)])?;
                            } else if subject6 == 4 {
                                ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(40)])?;
                            } else if subject6 == 5 {
                                ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(91)])?;
                            } else if subject6 == 6 {
                                ctx.call(Function::GetExperience, vec![Val::from(0), Val::from(151)])?;
                            }
                        }
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "The Kafra Corporation",
                                "provides our valued customers with a convenient Teleport Service which greatly cuts down on your",
                                "traveling time."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["Our Teleport Service is safe and comfortable, and will allow you to fully explore the various lands of the Midgard continent."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "We thank our valued customers for their great support and continue to provide them with the best",
                                "of service."
                            ],
                        )?;
                        ctx.next()?;
                    }
                    4 => {
                        ctx.lines_as(
                            "Kafra Employee",
                            args![
                                "The Kafra Corporation",
                                "provides a Cart Rental Service to Merchants, as well as Blacksmiths and Alchemists."
                            ],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["The flamboyantly mysterious", "^CE6300Super Novice^000000 can use Carts, but we officially don't have a contract with that class. Still, somehow..."])?;
                        ctx.next()?;
                        ctx.lines_as("Kafra Employee", args!["Anyway, Merchants, Blacksmiths and Alchemists must also learn the ^3355FFPush Cart^000000 skill in order to be able to rent a cart."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Kafra Employee",
                            args!["The Cart Rental service", "charge will differ from", "town to town."],
                        )?;
                        ctx.next()?;
                    }
                    5 => {
                        ctx.lines_as("Kafra Employee", args!["Thank you."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn kafra_employee_nv1(ctx: &Ctx) -> Script {
    kafra_employee_nv1_body(ctx, Vec::new()).map(|_| ())
}

fn instructor_nv_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Edwin",
        args!["Welcome to my class.", "Choose the subject you", "wish to learn more about."],
    )?;
    ctx.next()?;
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Basic Info Window:Party Window:Item Window:Option Window:Equipment Window:Cancel",
                )],
            )? {
                1 => {
                    ctx.lines_as("Edwin", args!["Let's take a look at", "the Basic Info Window,", "shall we?"])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "Your name, job, Basic Level,",
                            "and your Job Level are displayed in this window. ^800FFFBase level^000000 is your character's level."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "The ^800fffJob Level^000000 shown under the",
                            "Base Level displays the Job level of your character. When you just start a job, you will be at",
                            "Job Level 1."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args!["In the Basic Info window, your current experience points are displayed in the Base Level experience bar."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["Experience points in Ragnarok Online are indicated by percentage. Base and Job experience are separate from each other."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "When the Base Level or",
                            "Job Level bar reaches 100 it will be raised by one level and the bar will then reset to 0 for the next level."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["HP stands for 'health points.'", "When your HP is reduced to 0, you will faint and be unable to fight. You can either return to your spawn point, or wait for someone to revive you."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "If you die in the fields or dungeons, you will receive",
                            "a ^FF0000-1 % EXP penalty^000000, so be careful."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["SP stands for 'spell points.'", "When you become a 1st class, you will learn unique class skills of the class that will require SP to use. Your skill instructor can teach you more about skills."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "If you want to check the weight limit of the items you can carry, check the bottom left of the",
                            "Basic Info window."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args!["The current weight of the items you are carrying will be displayed next to your maximum weight limit."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["If you carry over 50 % of your maximum weight limit, your HP and SP will not be restored by resting, so be careful."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["The numerical value next to the weight limit shows the current amount of Zeny, the currency of Midgard, that you possess."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["On the right side of the", "Basic Info Window is a series of buttons that will open other interface windows. Click them one by one, and check what you can do with them."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "The shortcut for minimizing and maximizing the Basic Info Window",
                            "is '^3355FFAlt^000000 + ^3355FFV^000000.'"
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as("Edwin", args!["You can open the Party Window", "by clicking the ^3355FFfriend^000000 button in the Basic Info window. The shortcut for the Party Window is '^3355FFAlt^000000 + ^3355FFZ^000000.' You can use this window to check on the members of your party."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["The Party master can determine", "the distribution of EXP and items to the party. You can also check the location of your party members on the Mini-Map."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["In the Party Window, you can click on the ^3355FFFriend^000000 button to see your Friend List. You can use the", "Friend List to send whispers", "to your friends."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["You can ask your skill instructor to learn more about organizing parties. But I guess you can also just try that on your own."])?;
                    ctx.next()?;
                }
                3 => {
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "The item window is divided into",
                            "3 sections: consumable items, equipment and other items."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["Your weight limit does limit the amount of items you can carry with you. When you're carrying too many things, place your extra stuff in Kafra Storage."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args!["Also, equipment and consumable items can also be assigned to a Hotkey through the Hotkey bar."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "The Hotkey window is opened by pressing ^3355FFF12^000000 key. The F1 to F9 keys are the designated hotkeys."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "There are total of 3 sets",
                            "of Hotkeys. You can toggle",
                            "between Hotkey sets by",
                            "pressing the F12 button."
                        ],
                    )?;
                    ctx.next()?;
                }
                4 => {
                    ctx.lines_as("Edwin", args!["You can open the Option Window", "by pressing the ^3355FFoption^000000 button in the Basic Info window. You can also press the '^3355FFAlt^000000 + ^3355FFO^000000' keys as well."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args!["In the Option Window, you can adjust sound, GUI skin, and the transparency of the skin."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["With the sound button, you can turn the background music on or off, as well as adjust the volume. The same can be done for the sound effects."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["With the Skin option, you can change the GUI skin for the in-game windows. Scroll through the list of the skins you have and choose", "a skin."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "You can also download official skins from our official website:",
                            "^0000FFhttp://iro.ragnarokonline.com^000000"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["The Snap option allows your mouse cursor to automatically be placed on a target once it hovers within the target's vicinity."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["When you click to attack, your mouse cursor will automatically change into a sword shape. Skill and item targeting also work with the Snap function."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["It might be useful or awkward if you're not used to it. But once you're familiar with the Snap function, you will be able to adjust your own snap options."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Edwin",
                        args!["Well, it all depends on your experience. That's all for the Option Window."],
                    )?;
                    ctx.next()?;
                }
                5 => {
                    ctx.lines_as(
                        "Edwin",
                        args![
                            "Click the ^3355FFequip^000000 button",
                            "in your Basic Info Window,",
                            "or just press the '^3355FFAlt^000000 + ^3355FFQ^000000' keys."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["In the Equipment Window,", "you will see the items currently equipped on your character. In the very beginning, every character is equipped with a Knife and", "a Cotton Shirt."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["There are 2 ways to change your equipment. Double-click equipment in the Inventory Window or click and drag an item into the", "Equipment Window."])?;
                    ctx.next()?;
                    ctx.lines_as("Edwin", args!["You can also assign equipment to Hotkeys. This can be down when you drag equipment from the Inventory into the Hotkey Window.", "('^3355FFF12^000000' key)."])?;
                    ctx.next()?;
                }
                6 => {
                    ctx.lines_as("Edwin", args!["If you have", "any questions,", "feel free to ask me~"])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    Ok(Val::from(0))
}

pub fn instructor_nv(ctx: &Ctx) -> Script {
    instructor_nv_body(ctx, Vec::new()).map(|_| ())
}

fn somatology_instructor_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Jare Riotte",
        args![
            "Welcome, new adventurer.",
            "I, Jare Riotte will help you to learn about the fundamental",
            "rules of your Character Statuses."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Jare Riotte",
        args![
            "Also known as 'Stats,'",
            "your statuses are the fundamental building blocks of your character."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Jare Riotte", args!["I am willing to help you learn about the statuses more than anything else, so feel free to ask me about character statuses you", "may be wondering about."])?;
    ctx.next()?;
    ctx.lines_as(
        "Jare Riotte",
        args![
            "In Ragnarok Online,",
            "the Character Statuses are Strength, Agility, Vitality, Intelligence, Dexterity and Luck."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Jare Riotte", args!["I want my class to proceed", "according to your personal needs, so ask about the Status you wish to better understand. First, open your Status Window by using the '^3355FFAlt^000000 + ^3355FFA^000000' keys."])?;
    ctx.next()?;
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            match runtime::select_values(
                ctx,
                &[Val::from(
                    "Strength and Agility:Vitality and Intelligence:Dexterity and Luck:I do not wish to continue.",
                )],
            )? {
                1 => {
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Let me explain Strength first.",
                            "^4D4DFFStrength (STR)^000000 increases",
                            "^4D4DFFphysical attack damage (ATK) ^000000",
                            "and your ^4D4DFF maximum weight limit^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["1 STR increases your physical attack damage by 1 point. A certain amount of attack damage bonus is given when the STR stat is increased to a multiple of 10."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["The way the attack damage bonus is calculated is by taking the total strength value, removing the very last digit, and squaring the number you have left."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Let's say your attack is displayed as '48 + 1.' That's a total of 49 strength. When you remove the last digit, '9,' we are left with the number '4.' 4 multiplied by itself equals 16."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["So the physical attack bonus is 16. Now, if you had a total of 50 STR, the attack bonus would be 25. And if your STR is 100, your attack bonus would be 100."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Okay, now let's move", "on to Agility (AGI).", "Agility affects your Flee Rate and Attack Speed. The higher your Flee Rate, the better chance you have of avoiding enemy attacks."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Your Flee Rate",
                            "is equal to the number",
                            "of your Base Level added",
                            "to your AGI."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["So if you have 40 AGI and you're at Base Level 30, your Flee Rate would be 70. It's so simple! Following the Flee Rate formula, your flee rate will be a total of 70."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Now, AGI only relates to normal Flee Rate. Perfect Dodge is another factor that determines success in dodging attacks, but we'll talk about that when we discuss", "the LUK stat."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Another benefit to increasing AGI is that your Attack Speed (ASPD) will also increase, meaning the time between your melee attacks will be reduced."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args!["ASPD, however,", "differs by Job Class,", "so please remember that."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "^666666*Whew!*^000000",
                            "That's almost too",
                            "much excitement for",
                            "one day. Shall we move",
                            "on to the next subject?"
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Jare Riotte",
                        args!["Our next subject", "will be Vitality (VIT)", "and Intelligence (INT)."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["^4D4DFFVitality^000000 affects the ^4D4DFFmaximum HP, amount of HP restoration and defense^000000. The amount of HP increased by VIT is differs by your job class."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Out of all the Job Classes, the Swordman class benefits most",
                            "from increases in VIT."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Let's check defense.", "On your stat window, your defense is displayed as DEF. Two numerical values are shown, and the second number reflects the addition to your defense by your VIT."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Now, the first number",
                            "displayed in your Defense is the defense calculated from your Equipment and Armor."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Now, the equipment you wear reduces damage from enemies by a percentage of the total damage, where VIT reduces by a set amount. That's why Defense is displayed with two numbers."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["The next subject", "is Intelligence (INT)."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["INT affects your maximum amount of ^4D4DFFSP^000000, ^4D4DFFSP restoration^000000, the ^4D4DFF damage of your magic attack (MATK)^000000 and your ^4D4DFFdefense against magic attack (MDEF)^000000."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["The SP amount and MATK increased", "by 1 INT is dependent on Job Class, just like VIT. This means that some Jobs will naturally benefit more from having more INT than others."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Just like normal DEF, your Magic Defense (MDEF) shows as 2 different numerical values. The MDEF contributed by INT is the second MDEF value displayed."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["In order to study manipulation of the natural elements, you should prioritize on having intelligence. Therefore, Sages and Wizards", "focus on the INT stat."])?;
                    ctx.next()?;
                }
                3 => {
                    ctx.lines_as(
                        "Jare Riotte",
                        args!["Our last subject", "is Dexterity (DEX)", "and Luck (LUK)."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Dexterity mainly affects your accuracy, attack speed (ASPD)",
                            "and your average attack strength."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Let me explain...",
                            "If you have low DEX, the difference between the minimum damage and",
                            "the maximum damage you can inflict becomes huge. The damage of your attacks becomes unstable."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["If you are using a ^4D4DFFBow^000000 as your", "main weapon, your attack strength will be based on^4D4DFFDEX^000000. So Archers should focus on increasing their DEX."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["The amount of DEX that you", "have will also affect your attack accuracy. Attack accuracy is calculated by the number of your Base Level added to your DEX."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "For example, if you are at",
                            "Base Level 40 and have 20 DEX,",
                            "your attack accuracy would be 60."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Finally, DEX also reduces the casting time of spells and skills. Therefore, having some DEX would",
                            "be handy for Mages and Wizards."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Now, let me tell", "you about the LUK stat."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Luck (LUK) affects the chance",
                            "for a critical attack, the Flee Rate and a small amount of damage you deal to monsters."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["For a critical attack, the start value is 1 for everyone and it's increased by^4d4dff 1^000000 for every ^4d4dff3 LUK^000000."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["With more LUK, you have an increased change of inflicting ^3355FFcritical attacks^000000 to your enemies. Critical attacks are useful to you, as they pierce your enemy's defense."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["You can tell you've performed a critical attack when an attack has inflicted more damage than usual to an enemy, and the damage number is displayed with an explosive red visual effect."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["For every 10 LUK, you add 1 point to your Perfect Dodge rate. Perfect Dodge is sort of like your Flee Rate. When you perform a Perfect Dodge, the word '^FF7F00Lucky^000000' will appear over your head."])?;
                    ctx.next()?;
                    ctx.lines_as("Jare Riotte", args!["Although similar to your Flee Rate, Perfect Dodge is a separate factor in attack evasion that is calculated differently."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Jare Riotte",
                        args![
                            "Well, I must say,",
                            "Luck is a good thing to have, but that doesn't mean you need it before everything else."
                        ],
                    )?;
                    ctx.next()?;
                }
                4 => {
                    ctx.lines_as(
                        "Jare Riotte",
                        args!["Do you have any", "other questions?", "I hope my class", "was helpful to you."],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    Ok(Val::from(0))
}

pub fn somatology_instructor(ctx: &Ctx) -> Script {
    somatology_instructor_body(ctx, Vec::new()).map(|_| ())
}

fn understandings_of_skills_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    let mut l_end = Val::from(0);
    ctx.lines_as("Leo Handerson", args!["Welcome~", "Oh look at this", "cute little Novice~!"])?;
    ctx.next()?;
    ctx.lines_as("Leo Handerson", args!["I, Leo Handerson,", "feel so responsible for your performance and will be teaching you to the best of my ability. Now, please select the subject you wish to learn."])?;
    ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
    ctx.next()?;
    'l1: loop {
        if !(true) {
            break 'l1;
        }
        'b1: {
            match runtime::select_values(
                ctx,
                &[Val::from("Passive and Active skills:Basic Skills:Use of Emoticons:Cancel.")],
            )? {
                1 => {
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "First, let me explain",
                            "about the Passive Skills.",
                            "Would please you open",
                            "your Skill Window?",
                            "('^3355FFAlt^000000' + '^3355FFS^000000')"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "Now, you know that whenever",
                            "your Job Level goes up, you earn",
                            "a ^3355FFSkill Point^000000, right? Skill Points are used to learn your skills~"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["Do you see your Basic Skill icon? It's at the very top of the Skill Window. Click on the 'Lv Up' button next to the Basic Skill icon to use a Skill Point on Basic Skills."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "You'll see to the word 'Passive'",
                            "to the right of the Basic Skill icon. That means this skill is Passive, and doesn't use any SP."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["Now, please right-click the Basic Skill icon. You will then be able to read a brief description of the Basic Skills."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "Active Skills, unlike Passive Skills which don't use any SP, require SP each time that",
                            "they are used."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args!["You can use an Active Skill by double-clicking its icon in your Skill Window."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["You can also drag a skill icon from your Skill Window, and drop it into your Hotkey bar ('^3355FFF12^000000' key) to assign a Hotkey to that skill."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "The amount of SP required",
                            "to use an Active Skill will be displayed to the right of that skill's icon."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["Generally, Passive Skills are skills related to mental or physical training and conditioning. The use of special abilities or attacks are Active Skills."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "If you're still not",
                            "sure about my lesson,",
                            "I'm willing to go over",
                            "it once again."
                        ],
                    )?;
                    ctx.next()?;
                }
                2 => {
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "The Basic Skills are",
                            "purely Passive Skills",
                            "that you need to play",
                            "Ragnarok Online.",
                            "Don't worry,",
                            "they're easy to learn."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["When you right-click with your mouse in your Skill Window", "('^3355FFAlt^000000' + '^3355FFS^000000'), you can check the descriptions of the skills, but I've prepared this lesson for", "your better understanding."])?;
                    ctx.next()?;
                    'l3: loop {
                        if !(true) {
                            break 'l3;
                        }
                        'b3: {
                            match runtime::select_values(
                                ctx,
                                &[Val::from(
                                    "Trade and Exchange:Organizing & Joining party:Opening Chat Room:Storage Use:No thanks, I know this already.",
                                )],
                            )? {
                                1 => {
                                    ctx.lines_as("Leo Handerson", args!["When you go visit our official website at ^0000FFiro.ragnarokonline.com^000000, you can find a full explanation about trading, illustrated with pictures."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "In order to trade items",
                                            "or zeny with other people,",
                                            "you must learn at least ^3355FFBasic Skill Level 1^000000."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["When you trade, you must be", "located close to the person with which you wish to exchange items or zeny. Otherwise, the trade will not work if that person is more than 2 cells away."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "Right-click on the person",
                                            "once and a small menu will appear.",
                                            "From this menu, choose:",
                                            "^800fffRequest a deal with^000000."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["Afterwards, that person will", "choose whether or not to accept your request. If your trade request is accepted, the Trade Window will appear."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["To trade items, drag items from your Inventory Window ('^3355FFAlt^000000' + '^3355FFE^000000') and drop them into the left side of the Trade Window."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["Items given by the other person will appear in the right side of the Trade Window. Always check", "if the other person is trading the items you have agreed to exchange."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["To trade Zeny, you can enter the amount of Zeny you want to trade. After placing items or Zeny into the Trade Window, press the 'OK' button at the bottom of the Window to confirm the trade."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["Once the trade is confirmed,", "press the 'Trade' button to finish the trade. If either of the traders do not press the 'OK' button, the trade cannot be completed."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["If either of the traders presses the 'Cancel' button to the right of the window, the trade will be cancelled."])?;
                                    ctx.next()?;
                                }
                                2 => {
                                    ctx.lines_as("Leo Handerson", args!["Now, let me explain", "about the Party System."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["The Party system allows", "you to organize a small group with people in order to cooperatively hunt monsters, or just to have fun together."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "You can organize",
                                            "a party by typing",
                                            "the command:",
                                            "^4F4FFF//organize ''Party Name''^000000",
                                            "in your Chat Window."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "Of course, you",
                                            "must be at least",
                                            "^4d4dffBasic Skill Level 7^000000",
                                            "or above to use the Party System."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["I could explain more about", "the distribution of items or the distribution of experience which party members have gained together, but it's that you try that out on your own later on."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "That's all for",
                                            "the Party System~",
                                            "An adorable Novice",
                                            "like you should pick",
                                            "on this really quickly~"
                                        ],
                                    )?;
                                    ctx.call(Function::Emotion, vec![ctx.constant("ET_CHUP")?])?;
                                    ctx.next()?;
                                }
                                3 => {
                                    ctx.lines_as("Leo Handerson", args!["When you have", "^4D4DFFLevel 4 Basic Skill^000000", "or above, you can open your own Chat Room. You can either click", "the ^3355FFchat^000000 button in the Basic Info Window or just press '^3355FFAlt^000000' + '^3355FFC^000000.'"])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "Once you open a Chat Room,",
                                            "you can check the chat room members' information by right-clicking on a character name."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "Also you can register that character as your friend in the same way. If you are the master",
                                            "of the room, you can change",
                                            "the room setup."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "While in the Chat Room,",
                                            "you cannot hear any chat from outside of the Chat Room. Please remember that, okay?"
                                        ],
                                    )?;
                                    ctx.next()?;
                                }
                                4 => {
                                    ctx.lines_as("Leo Handerson", args!["Kafra Employees of the", "Kafra Corporation are scattered throughout the world, providing their convenient services and Storage."])?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["In fact, there's a Kafra Employee just outside of this room. Talk to her, and she'll be more than happy to fully explain Kafra's Services."])?;
                                    ctx.next()?;
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "Anyway, you are",
                                            "allowed to use your",
                                            "^4d4dffpersonal Kafra Storage^000000",
                                            "at ^4D4DFFBasic Skill Level 6^000000 or above."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    ctx.lines_as("Leo Handerson", args!["When you carry too many items with you, your weight becomes too heavy and you won't be able to restore HP or SP by resting, or even fight monsters! So store what you don't need into Kafra Storage."])?;
                                    ctx.next()?;
                                }
                                5 => {
                                    ctx.lines_as(
                                        "Leo Handerson",
                                        args![
                                            "Oh, do you?",
                                            "As I expected,",
                                            "you're as smart",
                                            "as you are cute~!",
                                            "I, Leo, am very impressed."
                                        ],
                                    )?;
                                    ctx.next()?;
                                    l_end = Val::from(1);
                                }
                                _ => {}
                            }
                            if l_end.clone().is_true() {
                                break 'l3;
                            }
                        }
                    }
                    ctx.lines_as(
                        "Leo Handerson",
                        args!["Do you wish to", "learn more about", "a different subject?"],
                    )?;
                    ctx.next()?;
                }
                3 => {
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "^4d4dffEmoticons^000000 are commonly used",
                            "online for ^4D4DFFdisplaying your feelings^000000. It's a fun way of communicating!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "You must first be at",
                            "^4D4DFFBasic Skill Level 2^000000",
                            "or above to use emoticons."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "You can check the",
                            "Emotion icon List",
                            "('^3355FFAlt^000000' + '^3355FFL^000000') and click each icon to see the command to display",
                            "the emoticon."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["You can also register", "emoticons into your Shortcut List ('^3355FFAlt^000000' + '^3355FFM^000000') so you can just use a Shortcut to use an emoticon. This is also fully explained on our official website as well."])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Leo Handerson",
                        args![
                            "Ah~ I must say,",
                            "the honest expression",
                            "of one's feelings is essential",
                            "for a relationship."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["You can play", "Rock, Paper, Scissors", "by pressing the '^4D4DFFCtrl^000000' and the '^4D4DFF - ^000000,' '^4D4DFF = ^000000,' or '^4D4DFF \\ ^000000' keys."])?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["Of course, you can type ^4D4DFF//bawi^000000, ^4D4DFF//bo^000000 and ^4D4DFF//gawi^000000, which mean rock, paper, scissors in Korean, into your Chat Window."])?;
                    ctx.next()?;
                    ctx.lines_as("Leo Handerson", args!["To check out the commands for even more emoticons, type the command ^4d4dff//emotion^000000 into your Chat Window to see the list~"])?;
                    ctx.call(Function::Emotion, vec![ctx.constant("ET_THROB")?])?;
                    ctx.next()?;
                }
                4 => {
                    ctx.lines_as(
                        "Leo Handerson",
                        args!["I see...", "You don't need me", "anymore! Oh! They", "grow up so fast", "nowadays!"],
                    )?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
        }
    }
    Ok(Val::from(0))
}

pub fn understandings_of_skills(ctx: &Ctx) -> Script {
    understandings_of_skills_body(ctx, Vec::new()).map(|_| ())
}

fn guide_soldier_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Guide Soldier",
        args![
            "We Guide Soldiers provide location information at the entrance of every town. You can easily find us by our special uniforms."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Guide Soldier",
        args![
            "Whenever you visit a town",
            "for the first time, we would like to recommend that you check the locations of notable places in town with us."
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Guide Soldier", args!["If you wish to take an Informative class, please walk around and speak to the various tutors in these Training Grounds. Have a good day."])?;
    ctx.close_window()?;
    return Err(Stop::End);
}

pub fn guide_soldier_nv1(ctx: &Ctx) -> Script {
    guide_soldier_nv1_body(ctx, Vec::new()).map(|_| ())
}

fn helper_nv_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("nov_2nd_cos").get()?.number()? < 11 {
        ctx.lines_as(
            "Elmeen",
            args![
                "Congratulations!",
                "You have passed the 1st training course! Wow~ I guess now you understand a little bit more about Ragnarok Online."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as("Elmeen", args!["You will learn the fundamentals of actual battle through this class. If you did your best through the Informative class, you are supposed to have been given some starting equipment."])?;
        ctx.next()?;
        ctx.lines_as(
            "Elmeen",
            args![
                "Please check your",
                "equipment before you engaging in battle. Are you sure you've equipped all of your equipment, your weapons and armor?"
            ],
        )?;
        ctx.next()?;
        'b1: {
            let subject1 = Val::from(runtime::select_values(ctx, &[Val::from("Yes:No")])?);
            let mut matched1 = false;
            let no_case1 = !subject1.loosely_equals(&Val::from(1)) && !subject1.loosely_equals(&Val::from(2));
            if !matched1 && subject1.loosely_equals(&Val::from(1)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as(
                    "Elmeen",
                    args!["First, you place the cursor on a monster. When you left click, you will hit the monster once."],
                )?;
                ctx.next()?;
                ctx.lines_as("Elmeen", args!["If you are too lazy to keep left clicking, left click on the monster while holding the '^4D4DFFCtrl^000000' key. You will then continue attacking the monster until one of you is dead, or you run away."])?;
                ctx.next()?;
                ctx.lines_as("Elmeen", args!["You can also just hold down the left mouse button while the cursor is on the monster. Still, there are some people who are even too lazy to use the '^4D4DFFCtrl^000000' key every time they attack a monster."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Elmeen",
                    args![
                        "If you're one of them, type the command ^E79E29//nc^000000 in your Chat Window. Then, when you left click",
                        "a monster, you'll just continuously attack it!"
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Elmeen",
                    args![
                        "If a monster happens to have the Undead property, you can use the 'Heal' skill to attack if you happen to have it."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Elmeen", args!["When you use the Heal skill while holding down the '^4D4DFFShift^000000' key, you can target the monster with the Heal skill to damage it."])?;
                ctx.next()?;
                ctx.lines_as("Elmeen", args!["Of course for this skill, we do have a very convenient option for lazy people too. Type the command ^E79E29//ns^000000 in your Chat Window."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Elmeen",
                    args!["This will allow you to attack monsters by using the Heal skill without holding the shift key. Handy, huh?"],
                )?;
                ctx.next()?;
                ctx.lines_as("Elmeen", args!["Do you understand these battle commands? Now, I will teaching you about monster behaviors and properties, experience gained through battle, and items you can earn from dead monsters."])?;
                ctx.next()?;
                ctx.lines_as(
                    "Elmeen",
                    args![
                        "Field Combat Training can be actually be pretty dangerous for new adventurers. Let me give you",
                        "a little more strength through the power of my magic."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Elmeen", args!["Haaaaaaa~!"])?;
                if ctx.var("nov_2nd_cos").get()?.number()? < 1 {
                    if ctx.var("BaseLevel").get()? == 1 {
                        ctx.var("nov_2nd_cos").set(Val::from(12))?;
                        ctx.call(Function::GetExperience, vec![Val::from(9), Val::from(0)])?;
                    } else {
                        if ctx.var("BaseLevel").get()? == 2 {
                            ctx.var("nov_2nd_cos").set(Val::from(13))?;
                            ctx.call(Function::GetExperience, vec![Val::from(16), Val::from(0)])?;
                        } else {
                            if ctx.var("BaseLevel").get()? == 3 {
                                ctx.var("nov_2nd_cos").set(Val::from(14))?;
                                ctx.call(Function::GetExperience, vec![Val::from(25), Val::from(0)])?;
                            } else if ctx.var("BaseLevel").get()? == 4 {
                                ctx.var("nov_2nd_cos").set(Val::from(15))?;
                                ctx.call(Function::GetExperience, vec![Val::from(36), Val::from(0)])?;
                            } else if ctx.var("BaseLevel").get()? == 5 {
                                ctx.var("nov_2nd_cos").set(Val::from(16))?;
                                ctx.call(Function::GetExperience, vec![Val::from(77), Val::from(0)])?;
                            } else if ctx.var("BaseLevel").get()? == 6 {
                                ctx.var("nov_2nd_cos").set(Val::from(17))?;
                                ctx.call(Function::GetExperience, vec![Val::from(112), Val::from(0)])?;
                            } else if ctx.var("BaseLevel").get()?.number()? >= 7 {
                                ctx.var("nov_2nd_cos").set(Val::from(18))?;
                                ctx.call(Function::GetExperience, vec![Val::from(153), Val::from(0)])?;
                            }
                        }
                    }
                }
                ctx.next()?;
                ctx.lines_as("Elmeen", args!["Which subject", "should I cover", "first for you?"])?;
                ctx.next()?;
                'l2: loop {
                    if !(true) {
                        break 'l2;
                    }
                    'b2: {
                        match runtime::select_values(ctx, &[Val::from("Monsters:Experience:Items:Quit.")])? {
                            1 => {
                                ctx.lines_as(
                                    "Elmeen",
                                    args!["There are many aggressive monsters that will attack you first before you even approach them. "],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["There are also a few monsters that will cooperate with others of their kind. Attack one of them, and the whole pack of them will swarm around you, seeking revenge."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Every monster can be specified by their types, sizes and properties. For example, there are Demi-human, Brute, Holy and Demon property monsters out there."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elmeen",
                                    args![
                                        "When you're aware of what property a monster is, you can use that knowledge to help you in battle."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["If you manage to get cards for that increase your damage upon certain monster properties, or reduce damage from specific monster properties, you'll have a much easier time in battle."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Monsters are separated by their size: small, medium and large. There are a few cards that allow you to do more damage to", "a specific monster size."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Also, each weapon has its strengths and weaknesses. The size of the weapon will affect the damage it will deal to monsters."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["For example, Dagger class weapons do 100 % damage on small sized monsters but only inflict 50 % on large monsters."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Common monster properties include Water, Wind, Earth, Fire, Shadow, Ghost and Holy. If you attack a monster with an opposing property, you can inflict additional damage~"])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["But if you attack a monster with", "a skill or weapon that inflicts damage of the same property as the monster, the damage will be greatly reduced, or completely negated. You might even heal the monster!"])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["In the case of Ghost property monsters, normal weapons cannot do any harm. However, a weapon with any other property will be able to deal out some damage."])?;
                                ctx.next()?;
                            }
                            2 => {
                                ctx.lines_as("Elmeen", args!["Basically, a character who deals the most damage on a monster receives the most experience points from the monster."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Therefore, you receive a certain percentage of experience points in proportion to the damage you've inflicted on the monster, compared to its total HP."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Let's say, there is a character named 'Z.' Z does 65 damage on a monster that has 100 total HP and gives 1000 experience points when it's dead. So, Z will receive 650 experience points."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elmeen",
                                    args!["However, this rule applies differently following certain situations."],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["If there are two people who both did 65 damage on the same monster, the experience points that each will receive differs, depending on the monster's remaining HP."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["For instance, somebody does damage to a monster while you're already hitting it, and he did the same amount of damage you did."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["In this case, you will receive 2//3 of the whole experience points that monster can give you, the other one will receive 1//3."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["However, attacking a monster that somebody already started to hit is not suggested in Ragnarok Online. That action is regarded as ill-mannered behavior."])?;
                                ctx.next()?;
                                ctx.lines_as(
                                    "Elmeen",
                                    args![
                                        "For party play, the party master can set the experience distribution to the equally share option."
                                    ],
                                )?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["With this method, party members can share their experience according to the their character levels, and the number of members in the party."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Also, there is the experience benefit for party play which allows you to gain more experience points than playing solo. You can take advantage of this system for faster leveling."])?;
                                ctx.next()?;
                            }
                            3 => {
                                ctx.lines_as("Elmeen", args!["When you kill monsters,", "you can obtain items by chance. Furthermore, certain characters can use the 'Steal' skill in order to steal items from monsters."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["Don't you worry about the Steal skill causing you to not find any items after you kill them. Using the Steal skill does not at all affect the item drop rate for monsters once they're killed."])?;
                                ctx.next()?;
                                ctx.lines_as("Elmeen", args!["When a group of people kill a monster, the person who did the most damage receives priority in picking up item drops."])?;
                                ctx.next()?;
                            }
                            4 => {
                                ctx.lines_as("Elmeen", args!["Feel free to", "ask me if you", "have any questions."])?;
                                ctx.close_window()?;
                                return Err(Stop::End);
                            }
                            _ => {}
                        }
                    }
                }
            }
            if !matched1 && subject1.loosely_equals(&Val::from(2)) {
                matched1 = true;
            }
            if matched1 {
                ctx.lines_as("Elmeen", args!["Please take care of the equipment you've received through the training courses. Once you lose the equipment, you can never get them back."])?;
                ctx.close_window()?;
                return Err(Stop::End);
            }
        }
    } else {
        ctx.lines_as("Elmeen", args!["Which subject", "shall I expain?"])?;
        ctx.next()?;
        'l4: loop {
            if !(true) {
                break 'l4;
            }
            'b4: {
                match runtime::select_values(ctx, &[Val::from("Monsters:Experience:Items:Quit.")])? {
                    1 => {
                        ctx.lines_as(
                            "Elmeen",
                            args!["There are many aggressive monsters that will attack you first before you even approach them. "],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["There are also a few monsters that will cooperate with others of their kind. Attack one of them, and the whole pack of them will swarm around you, seeking revenge."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Every monster can be specified by their types, sizes and properties. For example, there are Demi-human, Brute, Holy and Demon property monsters out there."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elmeen",
                            args!["When you're aware of what property a monster is, you can use that knowledge to help you in battle."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["If you manage to get cards for that increase your damage upon certain monster properties, or reduce damage from specific monster properties, you'll have a much easier time in battle."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Monsters are separated by their size: small, medium and large. There are a few cards that allow you to do more damage to a specific monster size."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Also, each weapon has its strengths and weaknesses. The size of the weapon will affect the damage it will deal to monsters."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["For example, Dagger class weapons do 100 % damage on small sized monsters but only inflict 50 % on large monsters."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Common monster properties include Water, Wind, Earth, Fire, Shadow, Ghost and Holy. If you attack a monster with an opposing property, you can inflict additional damage~"])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["But if you attack a monster with", "a skill or weapon that inflicts damage of the same property as the monster, the damage will be greatly reduced, or completely negated. You might even heal the monster!"])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["In the case of Ghost property monsters, normal weapons cannot do any harm. However, a weapon with any other property will be able to deal out some damage."])?;
                        ctx.next()?;
                    }
                    2 => {
                        ctx.lines_as("Elmeen", args!["Basically, a character who deals the most damage on a monster receives the most experience points from the monster."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Therefore, you receive a certain percentage of experience points in proportion to the damage you've inflicted on the monster, compared to its total HP."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Let's say, there is a character named 'Z.' Z does 65 damage on a monster that has 100 total HP and gives 1000 experience points when it's dead. So, Z will receive 650 experience points."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elmeen",
                            args!["However, this rule applies differently following certain situations."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["If there are two people who both did 65 damage on the same monster, the experience points that each will receive differs, depending on the monster's remaining HP."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["For instance, somebody does damage to a monster while you're already hitting it, and he did the same amount of damage you did."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["In this case, you will receive 2//3 of the whole experience points that monster can give you, the other one will receive 1//3."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["However, attacking a monster that somebody already started to hit is not suggested in Ragnarok Online. That action is regarded as ill-mannered behavior."])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Elmeen",
                            args!["For party play, the party master can set the experience distribution to the equally share option."],
                        )?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["With this method, party members can share their experience according to the their character levels, and the number of members in the party."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Also, there is the experience benefit for party play which allows you to gain more experience points than playing solo. You can take advantage of this system for faster leveling."])?;
                        ctx.next()?;
                    }
                    3 => {
                        ctx.lines_as("Elmeen", args!["When you kill monsters,", "you can obtain items by chance. Furthermore, certain characters can use the 'Steal' skill in order to steal items from monsters."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["Don't you worry about the Steal skill causing you to not find any items after you kill them. Using the Steal skill does not at all affect the item drop rate for monsters once they're killed."])?;
                        ctx.next()?;
                        ctx.lines_as("Elmeen", args!["When a group of people kill a monster, the person who did the most damage receives priority in picking up item drops."])?;
                        ctx.next()?;
                    }
                    4 => {
                        ctx.lines_as("Elmeen", args!["Feel free to", "ask me if you", "have any questions."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn helper_nv(ctx: &Ctx) -> Script {
    helper_nv_body(ctx, Vec::new()).map(|_| ())
}

fn entrance_guard_nv_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    if ctx.var("nov_2nd_cos").get()? == 0 {
        ctx.lines_as(
            "Muriel",
            args!["I'm sorry, but I can't let anybody who hasn't been instructed on fighting enter the Field Combat Training Grounds."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Muriel",
            args![
                "Why don't you speak to the Helper to the left side of this room first, so that you can receive some battle instruction?"
            ],
        )?;
        ctx.close_window()?;
        return Err(Stop::End);
    } else {
        if (ctx.var("nov_2nd_cos").get()?.number()? > 0 && ctx.var("nov_2nd_cos").get()?.number()? < 21) {
            ctx.lines_as("Muriel", args!["Field Combat Training is an actual fight class where you can gain basic fighting skills that you can use to defend yourself in Midgard."])?;
            ctx.next()?;
            ctx.lines_as(
                "Muriel",
                args!["Please kill as many monsters as you can to increase your base level at least 2 times."],
            )?;
            ctx.next()?;
            ctx.lines_as("Muriel", args!["Gaining 2 base levels is required to complete your Field Combat Training. Although the monsters are all weak and easy to kill, I hope you will be careful."])?;
            ctx.next()?;
            ctx.lines_as("Muriel", args!["Do you wish", "to take the test", "right away?"])?;
            ctx.next()?;
            match runtime::select_values(ctx, &[Val::from("Yes:I need more time.")])? {
                1 => {
                    ctx.lines_as(
                        "Muriel",
                        args![
                            "Please make sure you",
                            "talk to the staff at the North after you increase your base level by 2 levels through battle."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Muriel",
                        args!["I'm going to give you some useful supplies, so please use them in case of an emergency."],
                    )?;
                    if ctx.var("nov_2nd_cos").get()? == 12 {
                        ctx.var("nov_2nd_cos").set(Val::from(22))?;
                    } else {
                        if ctx.var("nov_2nd_cos").get()? == 13 {
                            ctx.var("nov_2nd_cos").set(Val::from(23))?;
                        } else {
                            if ctx.var("nov_2nd_cos").get()? == 14 {
                                ctx.var("nov_2nd_cos").set(Val::from(24))?;
                            } else if ctx.var("nov_2nd_cos").get()? == 15 {
                                ctx.var("nov_2nd_cos").set(Val::from(25))?;
                            } else if ctx.var("nov_2nd_cos").get()? == 16 {
                                ctx.var("nov_2nd_cos").set(Val::from(26))?;
                            } else if ctx.var("nov_2nd_cos").get()? == 17 {
                                ctx.var("nov_2nd_cos").set(Val::from(27))?;
                            } else if ctx.var("nov_2nd_cos").get()? == 18 {
                                ctx.var("nov_2nd_cos").set(Val::from(28))?;
                            } else {
                                ctx.var("nov_2nd_cos").set(Val::from(29))?;
                            }
                        }
                    }
                    ctx.call(Function::GetItem, vec![Val::from(602), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(601), Val::from(9)])?;
                    ctx.call(Function::GetItem, vec![Val::from(1243), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(2112), Val::from(1)])?;
                    ctx.call(Function::GetItem, vec![Val::from(611), Val::from(2)])?;
                    ctx.call(Function::GetItem, vec![Val::from(569), Val::from(300)])?;
                    ctx.close_window()?;
                    ctx.call(
                        Function::SavePoint,
                        vec![Val::from("new_1-2"), Val::from(23), Val::from(188), Val::from(1), Val::from(1)],
                    )?;
                    ctx.call(Function::Warp, vec![Val::from("new_1-3"), Val::from(96), Val::from(21)])?;
                    return Err(Stop::End);
                }
                2 => {
                    ctx.lines_as("Muriel", args!["No problem.", "If you're not sure if you can pass the test or not, why don't you go talk to the Helper to the left one more time? Please come back", "when you're ready."])?;
                    ctx.close_window()?;
                    return Err(Stop::End);
                }
                _ => {}
            }
            ctx.close_window()?;
            return Err(Stop::End);
        } else {
            if (ctx.var("nov_2nd_cos").get()?.number()? > 20 && ctx.var("nov_2nd_cos").get()?.number()? < 31) {
                ctx.lines_as(
                    "Muriel",
                    args![
                        "Oh well, I told you to be careful. Cheer up! It's not a big deal.",
                        " ",
                        "Failure teaches success.",
                        "You have many chances",
                        "to re-take the test."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Muriel", args!["Do you wish", "to try again?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes.:Can I have more time?")])? {
                    1 => {
                        ctx.lines_as("Muriel", args!["I will give you", "some supplies again.", "Please be careful!"])?;
                        if ctx.var("nov_2nd_cos").get()? == 22 {
                            ctx.var("nov_2nd_cos").set(Val::from(33))?;
                            ctx.call(Function::GetExperience, vec![Val::from(16), Val::from(0)])?;
                        } else {
                            if ctx.var("nov_2nd_cos").get()? == 23 {
                                ctx.var("nov_2nd_cos").set(Val::from(34))?;
                                ctx.call(Function::GetExperience, vec![Val::from(25), Val::from(0)])?;
                            } else {
                                if ctx.var("nov_2nd_cos").get()? == 24 {
                                    ctx.var("nov_2nd_cos").set(Val::from(35))?;
                                    ctx.call(Function::GetExperience, vec![Val::from(36), Val::from(0)])?;
                                } else {
                                    if ctx.var("nov_2nd_cos").get()? == 25 {
                                        ctx.var("nov_2nd_cos").set(Val::from(36))?;
                                        ctx.call(Function::GetExperience, vec![Val::from(77), Val::from(0)])?;
                                    } else if ctx.var("nov_2nd_cos").get()? == 26 {
                                        ctx.var("nov_2nd_cos").set(Val::from(37))?;
                                        ctx.call(Function::GetExperience, vec![Val::from(112), Val::from(0)])?;
                                    } else if ctx.var("nov_2nd_cos").get()? == 27 {
                                        ctx.var("nov_2nd_cos").set(Val::from(38))?;
                                        ctx.call(Function::GetExperience, vec![Val::from(153), Val::from(0)])?;
                                    } else if ctx.var("nov_2nd_cos").get()? == 28 {
                                        ctx.var("nov_2nd_cos").set(Val::from(39))?;
                                        ctx.call(Function::GetExperience, vec![Val::from(200), Val::from(0)])?;
                                    } else if ctx.var("nov_2nd_cos").get()? == 29 {
                                        ctx.var("nov_2nd_cos").set(Val::from(40))?;
                                        ctx.call(Function::GetExperience, vec![Val::from(200), Val::from(0)])?;
                                    }
                                }
                            }
                        }
                        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(0)])?;
                        ctx.call(Function::GetItem, vec![Val::from(569), Val::from(50)])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-3"), Val::from(96), Val::from(21)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Muriel", args!["No problem.", "If you're not sure if you can pass the test or not, why don't you go talk to the Helper to the left one more time? Please come back", "when you're ready."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            } else if ctx.var("nov_2nd_cos").get()?.number()? > 30 {
                ctx.lines_as(
                    "Muriel",
                    args![
                        "Oh well, I told you to be careful. Cheer up! It's not a big deal.",
                        " ",
                        "Failure teaches success.",
                        "You have many chances to re-take the test."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as("Muriel", args!["Do you wish to try again?"])?;
                ctx.next()?;
                match runtime::select_values(ctx, &[Val::from("Yes:Can I have more time?")])? {
                    1 => {
                        ctx.lines_as("Muriel", args!["I will restore", "your HP. Please", "be careful!"])?;
                        ctx.call(Function::PercentHeal, vec![Val::from(100), Val::from(0)])?;
                        ctx.close_window()?;
                        ctx.call(Function::Warp, vec![Val::from("new_1-3"), Val::from(96), Val::from(21)])?;
                        return Err(Stop::End);
                    }
                    2 => {
                        ctx.lines_as("Muriel", args!["No problem.", "If you're not sure if you can pass the test or not, why don't you go talk to the Helper to the left one more time? Please come back when you're ready."])?;
                        ctx.close_window()?;
                        return Err(Stop::End);
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(Val::from(0))
}

pub fn entrance_guard_nv(ctx: &Ctx) -> Script {
    entrance_guard_nv_body(ctx, Vec::new()).map(|_| ())
}

fn trainer_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Hoffman",
        args!["Hey there~", "I'm here to provide", "you with a little instruction."],
    )?;
    ctx.next()?;
    ctx.lines_as(
        "Hoffman",
        args![
            "These monsters are all weak",
            "and easy to kill. But be careful,",
            "a lot of them are aggressive",
            "and out for blood!"
        ],
    )?;
    ctx.next()?;
    ctx.lines_as("Hoffman", args!["If you think monsters here are too weak for you, I can send you to another training ground where the monsters are stronger than the ones over here."])?;
    ctx.next()?;
    ctx.lines_as(
        "Hoffman",
        args![
            "But don't worry so much,",
            "They're not impossible for",
            "Novices. So would you",
            "like to try?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(
        ctx,
        &[Val::from("I do want more of a challenge~:I wanna fight tough monsters!:Cancel")],
    )? {
        1 => {
            ctx.lines_as(
                "Hoffman",
                args![
                    "I see, then let me guide",
                    "you to a training ground that has stronger monsters. May God be with you..."
                ],
            )?;
            ctx.next()?;
            if ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "nv1" {
                if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                    ctx.call(Function::Warp, vec![Val::from("new_2-3"), Val::from(96), Val::from(21)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("new_3-3"), Val::from(96), Val::from(21)])?;
                }
            } else {
                ctx.call(Function::Warp, vec![Val::from("new_1-3"), Val::from(96), Val::from(21)])?;
            }
            return Err(Stop::End);
        }
        2 => {
            ctx.lines_as(
                "Hoffman",
                args![
                    "You must like ",
                    "rough challenges,",
                    "don't you? Please",
                    "be careful, it can get",
                    "pretty difficult..."
                ],
            )?;
            ctx.next()?;
            if ((ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "nv1"
                || ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "nv2")
                || ctx.call(Function::StrNpcInfo, vec![Val::from(2)])? == "nv3")
            {
                if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                    ctx.call(Function::Warp, vec![Val::from("new_4-3"), Val::from(96), Val::from(21)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("new_5-3"), Val::from(96), Val::from(21)])?;
                }
            } else {
                if ctx.call(Function::Rand, vec![Val::from(2)])?.is_true() {
                    ctx.call(Function::Warp, vec![Val::from("new_2-3"), Val::from(96), Val::from(21)])?;
                } else {
                    ctx.call(Function::Warp, vec![Val::from("new_3-3"), Val::from(96), Val::from(21)])?;
                }
            }
            return Err(Stop::End);
        }
        3 => {
            ctx.lines_as(
                "Hoffman",
                args![
                    "Hmm...?",
                    "Are you worried about going",
                    "to more challenging places? That's understandable, since you're still a new adventurer. Good luck~"
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn trainer_nv1(ctx: &Ctx) -> Script {
    trainer_nv1_body(ctx, Vec::new()).map(|_| ())
}

fn test_examiner_nv1_body(ctx: &Ctx, args: Vec<Val>) -> Result<Val, Stop> {
    ctx.lines_as(
        "Keyman",
        args![
            "Good!!",
            "Now you know how to fight",
            "against monsters, don't you?",
            "Would you like to move",
            "to the next course?"
        ],
    )?;
    ctx.next()?;
    match runtime::select_values(ctx, &[Val::from("Yes:No")])? {
        1 => {
            ctx.lines_as(
                "Keyman",
                args!["I hope you will be", "a good fighter in the", "future. Bon voyage."],
            )?;
            ctx.close_window()?;
            ctx.call(Function::Warp, vec![Val::from("new_1-4"), Val::from(99), Val::from(10)])?;
            return Ok(ctx.var("end").get()?);
        }
        2 => {
            ctx.lines_as(
                "Keyman",
                args![
                    "I see...",
                    "It can't hurt to practice until you're more comfortable with the basics of battle."
                ],
            )?;
            ctx.close_window()?;
            return Err(Stop::End);
        }
        _ => {}
    }
    Ok(Val::from(0))
}

pub fn test_examiner_nv1(ctx: &Ctx) -> Script {
    test_examiner_nv1_body(ctx, Vec::new()).map(|_| ())
}
