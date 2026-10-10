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

const CARDS_1: [i32; 127] = [
    4001, 4006, 4009, 4019, 4075, 4033, 4012, 4016, 4026, 4022, 4027, 4028, 4038, 4025, 4021, 4050, 4079, 4081, 4090, 4094, 4101, 4104,
    4110, 4114, 4119, 4108, 4095, 4231, 4280, 4008, 4011, 4013, 4014, 4015, 4020, 4032, 4037, 4039, 4041, 4045, 4046, 4010, 4023, 4029,
    4052, 4048, 4056, 4071, 4093, 4031, 4036, 4034, 4042, 4055, 4061, 4087, 4096, 4116, 4122, 4170, 4215, 4220, 4228, 4226, 4212, 4227,
    4267, 4257, 4278, 4286, 4287, 4292, 4311, 4315, 4319, 4322, 4084, 4078, 4113, 4149, 4153, 4196, 4240, 4247, 4256, 4057, 4066, 4067,
    4112, 4150, 4152, 4186, 4187, 4181, 4173, 4167, 4162, 4176, 4195, 4193, 4200, 4223, 4194, 4190, 4189, 4192, 4224, 4244, 4248, 4261,
    4260, 4259, 4274, 4275, 4313, 4299, 4304, 4294, 4076, 4127, 4154, 4157, 4156, 4213, 4214, 4225, 4235,
];

const CARDS_2: [i32; 127] = [
    4293, 4297, 4288, 4283, 4295, 4307, 4308, 4309, 4132, 4326, 4341, 4335, 4337, 4345, 4344, 4331, 4333, 4332, 4089, 4161, 4177, 4178,
    4180, 4184, 4191, 4206, 4199, 4273, 4282, 4268, 4289, 4321, 4316, 4343, 4339, 4369, 4377, 4385, 4383, 4382, 4380, 4381, 4378, 4379,
    4390, 4389, 4388, 4391, 4405, 4400, 4401, 4402, 4404, 4002, 4003, 4004, 4005, 4007, 4017, 4024, 4030, 4035, 4040, 4043, 4044, 4049,
    4051, 4053, 4058, 4060, 4062, 4063, 4064, 4065, 4068, 4069, 4070, 4072, 4073, 4074, 4077, 4080, 4082, 4083, 4085, 4086, 4088, 4091,
    4092, 4097, 4098, 4099, 4100, 4102, 4103, 4106, 4107, 4109, 4111, 4115, 4117, 4118, 4120, 4124, 4125, 4126, 4138, 4139, 4141, 4151,
    4158, 4164, 4165, 4182, 4185, 4159, 4160, 4166, 4172, 4175, 4188, 4201, 4202, 4204, 4205, 4208, 4209,
];

const CARDS_3: [i32; 121] = [
    4120, 4216, 4217, 4219, 4221, 4222, 4230, 4234, 4233, 4232, 4237, 4238, 4242, 4243, 4245, 4246, 4249, 4252, 4255, 4258, 4262, 4264,
    4276, 4270, 4271, 4218, 4239, 4251, 4253, 4269, 4334, 4105, 4133, 4136, 4229, 4272, 4277, 4279, 4281, 4284, 4285, 4290, 4296, 4298,
    4301, 4310, 4314, 4317, 4325, 4327, 4328, 4329, 4338, 4340, 4346, 4347, 4348, 4349, 4350, 4351, 4353, 4354, 4355, 4356, 4358, 4360,
    4362, 4364, 4366, 4368, 4370, 4371, 4373, 4375, 4387, 4406, 4129, 4155, 4291, 4392, 4393, 4394, 4409, 4410, 4411, 4412, 4413, 4414,
    4415, 4416, 4417, 4418, 4420, 4421, 4422, 4423, 4424, 4427, 4427, 4428, 4429, 4431, 4432, 4433, 4434, 4435, 4436, 4437, 4438, 4439,
    4440, 4442, 4443, 4444, 4445, 4447, 4448, 4449, 4450, 4452, 4453,
];

// (points cost, item id, amount, dialogue) for the item menu, in menu order
const PUTTY_REWARDS: [(i32, i32, i32, &[&str]); 4] = [
    (
        100,
        616,
        1,
        &[
            "Great, I wish you the best",
            "of luck with this album. I have a",
            "very good feeling about this one!",
        ],
    ),
    (
        50,
        607,
        20,
        &["Oh, you must like adventures.", "Here you go, just what you need!"],
    ),
    (
        20,
        505,
        10,
        &["Blue Potions? Are you sure?", "If that's what you want, here they are!"],
    ),
    (
        1,
        518,
        4,
        &["It took many bees", "to make all of this.", "Make good use of it."],
    ),
];

pub fn putty(ctx: &Ctx) -> Script {
    let mut card = Val::from(0);
    let mut cards_1: Vec<Val> = Vec::new();
    let mut cards_2: Vec<Val> = Vec::new();
    let mut cards_3: Vec<Val> = Vec::new();
    let mut count = Val::from(0);
    let mut card_counts: Vec<Val> = Vec::new();
    for (index, id) in CARDS_1.iter().enumerate() {
        runtime::local_set(&mut cards_1, &Val::from(index as i32), Val::from(*id), false);
    }
    for (index, id) in CARDS_2.iter().enumerate() {
        runtime::local_set(&mut cards_2, &Val::from(index as i32), Val::from(*id), false);
    }
    for (index, id) in CARDS_3.iter().enumerate() {
        runtime::local_set(&mut cards_3, &Val::from(index as i32), Val::from(*id), false);
    }
    runtime::local_set(&mut card_counts, &Val::from(1), Val::from(cards_1.len() as i32), false);
    runtime::local_set(&mut card_counts, &Val::from(2), Val::from(cards_2.len() as i32), false);
    runtime::local_set(&mut card_counts, &Val::from(3), Val::from(cards_3.len() as i32), false);
    let points = ctx.var("oversea_event9").get()?;
    if points.is_true() {
        ctx.lines_as(
            "Putty",
            args![
                "Welcome back!",
                "I see you already have some",
                Val::from("trading points. Actually, you currently have ^00cc00") + points.clone() + Val::from("^000000.")
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Putty",
            args![
                "Would you like to exchange these",
                "points now, or you would like to exchange more cards?"
            ],
        )?;
        ctx.next()?;
    } else {
        ctx.lines_as(
            "Putty",
            args![
                "Hi there.",
                "I don't know if I can be",
                "of any assistance, but I",
                "am trying to help older veterans",
                "by exchaning the cards that",
                "they are no longer using."
            ],
        )?;
        ctx.next()?;
        if ctx.menu(&["Ask for more information!", "I don't have any cards right now."])? == 0 {
            ctx.lines_as(
                "Putty",
                args![
                    "I am giving 1 point for each card that you bring me.",
                    "The points can be used to exchange for items that I have."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Putty",
                args![
                    "For ^CC0000100 points^000000: ^0000CC1 Old Card Album^000000.",
                    "For ^CC000050 points^000000: ^0000CC20 Yggdrasil Berry^000000.",
                    "For ^CC000020 points^000000: ^0000CC10 Blue Potion^000000.",
                    "For ^CC00001 point^000000: ^0000CC4 Honey^000000."
                ],
            )?;
            ctx.next()?;
        } else {
            ctx.lines_as("Putty", args!["Well, remember this offer!"])?;
            return ctx.close();
        }
    }
    match ctx.menu(&["I would like to exchange cards.", "Can I exchange the points?"])? {
        0 => {
            ctx.lines_as("Putty", args!["Please tell me what card you want to exchange."])?;
            ctx.next()?;
            let (typed, _) = runtime::input_text(ctx, None, None)?;
            let search = Val::from("_") + typed.clone();
            let mut group = Val::from(1);
            while group.number()? < 4 {
                let mut slot = Val::from(0);
                while slot.number()? < runtime::local_get(&card_counts, &group, false).number()? {
                    let name = Val::from(".@card") + group.clone() + Val::from("[") + slot.clone() + Val::from("]");
                    if runtime::compare(
                        &search,
                        &(Val::from("_")
                            + ctx.call(
                                Function::GetItemName,
                                args![runtime::getd(
                                    ctx,
                                    &name,
                                    &[
                                        (".@card", runtime::Local::Scalar(&card)),
                                        (".@card1", runtime::Local::Array(&cards_1)),
                                        (".@card2", runtime::Local::Array(&cards_2)),
                                        (".@card3", runtime::Local::Array(&cards_3)),
                                        (".@count", runtime::Local::Scalar(&count)),
                                        (".@i", runtime::Local::Scalar(&slot)),
                                        (".@i$", runtime::Local::Scalar(&typed)),
                                        (".@input$", runtime::Local::Scalar(&search)),
                                        (".@j", runtime::Local::Scalar(&group)),
                                        (".@points", runtime::Local::Scalar(&points)),
                                        (".@size_card", runtime::Local::Array(&card_counts)),
                                    ],
                                )?],
                            )?),
                    )
                    .is_true()
                    {
                        card = runtime::getd(
                            ctx,
                            &name,
                            &[
                                (".@card", runtime::Local::Scalar(&card)),
                                (".@card1", runtime::Local::Array(&cards_1)),
                                (".@card2", runtime::Local::Array(&cards_2)),
                                (".@card3", runtime::Local::Array(&cards_3)),
                                (".@count", runtime::Local::Scalar(&count)),
                                (".@i", runtime::Local::Scalar(&slot)),
                                (".@i$", runtime::Local::Scalar(&typed)),
                                (".@input$", runtime::Local::Scalar(&search)),
                                (".@j", runtime::Local::Scalar(&group)),
                                (".@points", runtime::Local::Scalar(&points)),
                                (".@size_card", runtime::Local::Array(&card_counts)),
                            ],
                        )?;
                        break;
                    }
                    slot = slot + Val::from(1);
                }
                ctx.call(Function::Sleep, args![10])?;
                if card.is_true() {
                    break;
                }
                group = group + Val::from(1);
            }
            ctx.mes("[Putty]")?;
            if !card.is_true() {
                ctx.mes("Please, come back here if you want to exchange a monster card.")?;
                return ctx.close();
            }
            count = ctx.call(Function::CountItem, args![card.clone()])?;
            if !count.is_true() {
                ctx.lines(args![
                    Val::from("You don't have any ^0055FF")
                        + shared::other_global_functions::f_getplural(ctx, args![ctx.call(Function::GetItemName, args![card.clone()])?])?
                        + Val::from("^000000 with you!")
                ])?;
                return ctx.close();
            }
            ctx.lines(args![
                Val::from("You've got ^0055FF")
                    + shared::other_global_functions::f_insertplural(
                        ctx,
                        args![count.clone(), ctx.call(Function::GetItemName, args![card.clone()])?]
                    )?
                    + Val::from("^000000."),
                " ",
                "Would you like to exchange 1 point for each of them?"
            ])?;
            ctx.next()?;
            match ctx.menu(&["Yes, please!", "No, thank you."])? {
                0 => {
                    ctx.call(Function::DelItem, args![card.clone(), count.clone()])?;
                    ctx.var("oversea_event9").set(points.clone() + count.clone())?;
                    ctx.lines_as(
                        "Putty",
                        args![Val::from("Alright, you have received ^CC0000") + count.clone() + Val::from("^000000 points.")],
                    )?;
                }
                _ => ctx.lines_as("Putty", args!["Okay, let me know if I can help you with something else."])?,
            }
            ctx.close()
        }
        _ => {
            if points.is_true() {
                ctx.lines_as("Putty", args!["These are the items that I have."])?;
                ctx.next()?;
                let choice = ctx.menu(&[
                    "1 Old Card Album - 50 Points",
                    "20 Yggdrasil Berry - 50 Points",
                    "10 Blue Potion - 20 Points",
                    "4 Honey - 1 Point",
                    "^777777Nerver mind.^000000",
                ])?;
                if choice == 4 {
                    ctx.lines_as("Putty", args!["Alright, come back when you have more points."])?;
                    return ctx.close();
                }
                let (cost, item, amount, text) = PUTTY_REWARDS[choice];
                if points.number()? >= cost {
                    ctx.lines_as("Putty", text.iter().map(|line| Val::from(*line)).collect())?;
                    ctx.var("oversea_event9").set(Val::from(points.number()? - cost))?;
                    ctx.call(Function::GetItem, args![item, amount])?;
                    return ctx.close();
                }
                ctx.lines_as("Putty", args!["Sorry, but you don't have enough points."])?;
                ctx.close()
            } else {
                ctx.lines_as("Putty", args!["You have 0 points. You need at least 1 point to exchange."])?;
                ctx.close()
            }
        }
    }
}
