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

pub fn utan_chief(ctx: &Ctx) -> Script {
    if ctx.var("event_umbala").get()? == 0 {
        ctx.lines_as(
            "Karkatan",
            args![
                "Huh huh, a Rune-Midgartsian.",
                "I guess this is your first",
                "visit to my village, isn't it?"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "Everyone from Rune-Midgarts",
                "that I've met had the same",
                "same expression on their",
                "face as you do right now",
                "when they first came here."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "Maybe it's because they cannot",
                "communicate with us due to",
                "the language barrier, so",
                "they have no idea what's going",
                "on. Yeah, I understand...",
                "Anyway, welcome to my village."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "My name is Karkatan, and I",
                "am the chief of the Utan tribe.",
                "You must be wondering how",
                "I can speak your language."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "It was taught to me long ago",
                "by an adventurer from your",
                "land. It's been a long time,",
                "and I do not know what has",
                "become of him..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args!["Anyhow, I learned many things", "about Rune-Midgartsian culture", "and language."],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "Sometimes, I teach the Utan",
                "language, but I do not give",
                "everyone that privilege.",
                "If unscrupulous outsiders",
                "learn the Utan language, they",
                "may bring harm to my tribe."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "Before you can learn the Utan",
                "language, first try to learn",
                "Utan culture by exploring our",
                "village."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "Although you are not able to",
                "communicate with my people",
                "right now, try to understand",
                "our way of life through your",
                "observations."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "Pay attention to the dress,",
                "appearance and life style of the",
                "local people. When you think",
                "you understand enough about Utan",
                "culture, come back to me and show me what you have learned."
            ],
        )?;
        ctx.var("event_umbala").set(Val::from(1))?;
        return ctx.close();
    } else if ctx.var("event_umbala").get()? == 1 {
        ctx.lines_as(
            "Karkatan",
            args![
                "Oh, it's you again. So...",
                "Have you learned about Utan",
                "culture? I want to hear your",
                "opinion, as well as your impression."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "There are still some villagers",
                "who are very naive about Rune-Midgartsians.",
                "Usually, they fear encounters",
                "with your people and will",
                "hide themselves."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Karkatan",
            args![
                "So...",
                "May I help you with anything?",
                "I assume you did not have much of",
                "a problem looking around the",
                "village, but it seems you have something to ask of me."
            ],
        )?;
        ctx.next()?;
        match ctx.menu(&["I want to learn Utan language.", "Umbabah Umbabah?", "Nothing."])? {
            0 => {
                if ctx.call(Function::IsEquipped, args![2278])?.is_true()
                    || ctx.call(Function::IsEquipped, args![2297])?.is_true()
                    || ctx.call(Function::IsEquipped, args![2288])?.is_true()
                    || ctx.call(Function::IsEquipped, args![2292])?.is_true()
                    || ctx.call(Function::IsEquipped, args![5005])?.is_true()
                    || ctx.call(Function::IsEquipped, args![2281])?.is_true()
                    || ctx.call(Function::IsEquipped, args![5043])?.is_true()
                {
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "Hmmm...That's an awesome mask",
                            "you're wearing. We Utans like",
                            "wearing masks to keep from",
                            "showing our facial expressions."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "That's why we wear masks all the",
                            "time. We believe that interaction",
                            "and treatment of other people",
                            "should not depend on how we look."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "Alright. I am sure you are",
                            "qualified to learn the Utan",
                            "language. I will teach you how",
                            "speak and to read in Utan from",
                            "now on."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "However, I need you to get some",
                            "items ready so that we may proceed",
                            "with the lessons. First, we need",
                            "two different kinds of paper.",
                            "^3377FF10 Oil Paper^000000 and ",
                            "^3377FF5 Slick Paper^000000."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "We'll also need something to",
                            "write with. Let's use",
                            "^3377FF1 Squid Ink^000000 and",
                            "^3377FF1 Feather of Birds^000000.",
                            "Please bring me those, and I will",
                            "teach you when you're ready."
                        ],
                    )?;
                    ctx.var("event_umbala").set(Val::from(2))?;
                    return ctx.close();
                } else {
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "You don't seem to understand",
                            "our culture yet. You cannot",
                            "learn another language if you",
                            "do not understand the culture."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "When you have that expression on",
                            "your face, Utans will be",
                            "intimidated... Since we do",
                            "not show our faces to others,",
                            "we are actually very",
                            "vulnerable to facial expression."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "Go explore the village a little",
                            "longer. You can come back",
                            "anytime when you think you're ready."
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Karkatan",
                        args![
                            "In any case, what do you think",
                            "about my mask? It's the current",
                            "trend among us Utans...don't you",
                            "think it's awesome?"
                        ],
                    )?;
                    return ctx.close();
                }
            }
            1 => {
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "Haha~ When you're just imitating",
                        "the sound, you won't make any",
                        "sense. Language is a mutual system",
                        "for the communication of thoughts and feelings."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "I regret to say that it seems that",
                        "nowadays, all peoples are no",
                        "longer sensitive to other cultures",
                        "in that respect."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "I see people that despise or",
                        "ridicule others that do not",
                        "understand them. It's really",
                        "sad that such bigotry still exists..."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "If you are interested in Utan",
                        "language, try to understand our",
                        "culture better and come back",
                        "when you're ready. I will",
                        "teach you the meanings of those sounds you are using."
                    ],
                )?;
                return ctx.close();
            }
            2 => {
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "Sometimes it's good to wander",
                        "without purpose. But it's",
                        "better to set a goal for",
                        "a journey if you want to",
                        "learn something out of",
                        "the experience."
                    ],
                )?;
                return ctx.close();
            }
            _ => {}
        }
    } else if ctx.var("event_umbala").get()? == 2 {
        if ctx.items().count(7151)? > 9 && ctx.items().count(7111)? > 4 && ctx.items().count(1024)? > 0 && ctx.items().count(916)? > 0 {
            ctx.lines_as(
                "Karkatan",
                args![
                    "Okay, I guess we're good to go.",
                    "Let's get the lesson started.",
                    "I hope you will communicate better",
                    "with Utans when we are done."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as("Karkatan", args![".............."])?;
            ctx.next()?;
            ctx.lines_as("Karkatan", args!["..............", "....................."])?;
            ctx.next()?;
            ctx.lines_as(
                "Karkatan",
                args!["..............", ".....................", "............................"],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Karkatan",
                args![
                    "Alright, that's all. Just forget",
                    "about how you've felt about Utans",
                    "before you learned the language.",
                    "Now go try to talk to Utans.",
                    "Conversation is a very important method in understanding others."
                ],
            )?;
            ctx.items().take(7151, 10)?;
            ctx.items().take(7111, 5)?;
            ctx.items().take(1024, 1)?;
            ctx.items().take(916, 1)?;
            ctx.var("event_umbala").set(Val::from(3))?;
            ctx.next()?;
            ctx.lines_as(
                "Karkatan",
                args![
                    "Okay, if you have any business",
                    "in our village later, feel free",
                    "to talk to me. I will try to help",
                    "you as much as I can."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Karkatan",
                args![
                    "I guess you are not ready yet...",
                    "Did you forget what items you",
                    "need? I will let you know",
                    "again, so please bring them",
                    "so that we can start the lesson."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Karkatan",
                args![
                    "^3377FF10 Oil Paper^000000,",
                    "^3377FF5 Slick Paper^000000,",
                    "^3377FF1 Squid Ink^000000,",
                    "^3377FF1 Feather of Birds^000000.",
                    "When you bring all of these,",
                    "I will teach you our language."
                ],
            )?;
            return ctx.close();
        }
    } else if ctx.var("event_umbala").get()?.number()? >= 3 {
        if ctx.var("event_umbala").get()? == 4 {
            ctx.lines_as(
                "Karkatan",
                args![
                    "Puchuchartan must have sent you to",
                    "me. I need to check whether or not",
                    "you are qualified to request",
                    "her help...We Utans do not want",
                    "to help evil people."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Karkatan",
                args!["Hmmmm....", "It would be good to have a mask", "that was made in Rune-Midgarts..."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Karkatan",
                args![
                    "I wish to have ^3377FF1 Mr. Smile^000000.",
                    "To Utans, receiving a mask as a",
                    "present is considered an",
                    "honor. Maybe Puchuchartan",
                    "wants you to show us your respect by doing so."
                ],
            )?;
            ctx.var("event_umbala").set(Val::from(5))?;
            return ctx.close();
        } else if ctx.var("event_umbala").get()? == 5 {
            if ctx.items().count(2278)? > 0 {
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "Oh, you brought it! Yes, I've",
                        "always wished that I could have",
                        "this mask! This is truly an",
                        "honor! Thank you, adventurer",
                        "from Rune-Midgarts."
                    ],
                )?;
                ctx.next()?;
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "I will tell Puchuchartan that I",
                        "confirmed your qualification.",
                        "Go and speak to her. Though I",
                        "am not sure what help she can give",
                        "you, I hope we will be able to return this favor."
                    ],
                )?;
                ctx.items().take(2278, 1)?;
                ctx.var("event_umbala").set(Val::from(6))?;
                return ctx.close();
            } else {
                ctx.lines_as(
                    "Karkatan",
                    args![
                        "Did I tell you that you need",
                        "^3377FF1 Mr. Smile^000000?",
                        "Please bring that as proof",
                        "of your goodwill, as well",
                        "as your sense of honor."
                    ],
                )?;
                return ctx.close();
            }
        } else {
            ctx.lines_as(
                "Karkatan",
                args![
                    "How's it going?",
                    "I wish I could guide you around",
                    "the village, but I cannot neglect",
                    "my duty as tribal chief."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Karkatan",
                args![
                    "Leading a tribe is not as easy",
                    "as it looks. You would understand",
                    "if you were in the same position",
                    "as me. Anyway, I hope you will enjoy your time in our village."
                ],
            )?;
            return ctx.close();
        }
    }
    Ok(())
}

pub fn utan_shaman(ctx: &Ctx) -> Script {
    let mut l_amount = Val::from(0);
    let mut l_consume = Val::from(0);
    let mut l_divide = Val::from(0);
    let mut l_input = Val::from(0);
    let mut l_input2 = Val::from(0);
    let mut l_sha_man = Val::from(0);
    let mut l_shaman_max = Val::from(0);
    let mut l_success = Val::from(0);
    if ctx.call(Function::CheckWeight, args![908, 600])? == 0 {
        ctx.lines(args![
            "^3355FFWait a second!",
            "Right now, you're carrying",
            "too many things with you.",
            "Please come back after",
            "using the Kafra Service",
            "to store some of your items.^000000"
        ])?;
        return ctx.close();
    }
    match ctx.var("event_umbala").get()?.number()? {
        3 => {
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "I did not expect that even",
                    "more of you Rune-Midgartsians",
                    "would find my village. I am",
                    "afraid that Mother Earth may",
                    "be caused suffering because",
                    "of this..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "You Rune-Midgartsians are a very",
                    "evil tribe...always accomplishing",
                    "your goals whether the means are",
                    "foul or fair, never hesitating to",
                    "ruin the property of others to get",
                    "what you want."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "I am worried how continuing",
                    "contact with the outside world",
                    "will affect our future..."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "No one from Rune-Midgarts has ",
                    "visited me without some purpose",
                    "and I do not think that you",
                    "are an exception."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "I only use my power for the",
                    "service of my tribe, and do not",
                    "give my aid to strangers. If you",
                    "really need my help, go ask for",
                    "the chief's permission."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "Also, I do not approve of",
                    "outsiders talking to the tribe",
                    "more than they have to...",
                    "We want to live a peaceful life, so do not disturb us."
                ],
            )?;
            ctx.var("event_umbala").set(Val::from(4))?;
            return ctx.close();
        }
        4 | 5 => {
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "I already told you to get the",
                    "chief's approval. There is also",
                    "the matter of my own business to take care of."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "I know that you have gone through",
                    "many difficulties to come here,",
                    "but you have to leave now."
                ],
            )?;
            return ctx.close();
        }
        6 => {
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "I've heard from the chief that he",
                    "has given you his approval...",
                    "Although I do not like this, I",
                    "will keep my promise. But it's",
                    "your call if you really need my help or not."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "My power allows me to create rough",
                    "enchanted stones and to divide a",
                    "pure enchanted stone into rough",
                    "ones. So I may be able to help",
                    "you in this way."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Puchuchartan",
                args!["So come and speak to me when", "you think my power may be", "of service to you."],
            )?;
            ctx.var("event_umbala").set(Val::from(7))?;
            return ctx.close();
        }
        7 => {
            ctx.lines_as(
                "Putsuchiritan",
                args![
                    "I don't know whether my talents",
                    "will be useful to you, but I'll",
                    "help you anyway."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Putsuchiritan",
                args![
                    "I can create elemental essence from natural objects,",
                    "or, dismantle elemental stones into their component essences.",
                    "Which would you like to do?"
                ],
            )?;
        }
        _ => {
            ctx.lines_as(
                "??????????",
                args![
                    "Umbah umbah umbabah Utan umbah",
                    "Umbah mookala umbabah..",
                    "Umbabahumbah umbabah",
                    "Umbabah umbaba umbaumbah umbah",
                    "Hum umbah umbah."
                ],
            )?;
            ctx.close_window()?;
            ctx.warp("umbala", 217, 186)?;
            return ctx.end();
        }
    }
    ctx.next()?;
    ctx.lines_as(
        "Puchuchartan",
        args![
            "Rune-Midgartsian who has asked for",
            "my help...Although I am not sure",
            "if you really need my power, I",
            "will try to provide my assistance."
        ],
    )?;
    ctx.next()?;
    if ctx.call(Function::CheckWeight, args![1101, 10])? == 0 {
        ctx.lines_as(
            "Puchuchartan",
            args![
                "Wait--!",
                "something in your possession",
                "is disturbing my peace of",
                "mind. This will not do..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Puchuchartan",
            args![
                "Go leave your belongings",
                "elsewhere, and only bring the",
                "items that you need right now."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Puchuchartan",
            args![
                "If you refuse to do so,",
                "I cannot do anything for you.",
                "Get yourself ready and then",
                "come back."
            ],
        )?;
        return ctx.close();
    }
    ctx.lines_as(
        "Puchuchartan",
        args![
            "Now, what do you wish to do?",
            "My power allows me to create rough",
            "enchanted stones and to divide a",
            "pure enchanted stone into rough ones."
        ],
    )?;
    ctx.next()?;
    match ctx.menu(&["Create rough enchanted stones", "Divide a pure enchanted stone", "Quit."])? {
        0 => {
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "Do you wish to create rough",
                    "enchanted stones? Which",
                    "property do you wish to create?",
                    "Earth, Water, Fire, Wind...",
                    "...choose one."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Earth", "Water", "Fire", "Wind"])? {
                0 => {
                    l_consume = Val::from(947);
                    l_amount = Val::from(15);
                    l_success = Val::from(993);
                }
                1 => {
                    l_consume = Val::from(946);
                    l_amount = Val::from(20);
                    l_success = Val::from(991);
                }
                2 => {
                    l_consume = Val::from(904);
                    l_amount = Val::from(20);
                    l_success = Val::from(990);
                }
                3 => {
                    l_consume = Val::from(1013);
                    l_amount = Val::from(25);
                    l_success = Val::from(992);
                }
                _ => {}
            }
            if ctx.call(Function::CountItem, args![l_consume.clone()])?.number()? >= l_amount.number()? {
                ctx.lines_as(
                    "Puchuchartan",
                    args![
                        "I will try to amplify the hidden",
                        "power of natural objects in",
                        "order to create rough enchanted",
                        "stones. Choose one number from",
                        "'1' to '9.' If you wish to cancel",
                        "this request, enter '0.'"
                    ],
                )?;
                ctx.next()?;
                'l4: loop {
                    let (input, _) = runtime::input_number(ctx, Some(0), Some(10))?;
                    l_input = input;
                    if l_input == 0 {
                        ctx.lines_as("Puchuchartan", args!["I see. It's your call.", "Come back when you need me."])?;
                        return ctx.close();
                    } else if l_input.number()? > 9 {
                        ctx.lines_as("Puchuchartan", args!["Remember to choose a number", "from 1 to 9."])?;
                        ctx.next()?;
                    } else {
                        break 'l4;
                    }
                }
                ctx.mes("[Puchuchartan]")?;
                match l_success.number()? {
                    990 => {
                        ctx.mes("I am putting these tails into a")?;
                    }
                    991 => {
                        ctx.mes("I am putting these shells into a")?;
                    }
                    992 => {
                        ctx.mes("I am putting these shells into a")?;
                    }
                    993 => {
                        ctx.mes("I am putting these horns into a")?;
                    }
                    _ => {}
                }
                ctx.lines(args![
                    "boiling pot, and casting a",
                    "sacred incantation. Remember",
                    "the number you entered."
                ])?;
                ctx.next()?;
                ctx.lines_as(
                    "Puchuchartan",
                    args![
                        "Amba Omba Zatumba! Umba! Ti!",
                        "Umputaun Eulukaba! Umba! Ha!",
                        "Julu Humba Rulala! Umba! La!",
                        "Datuha Ombabalaka! Umba! Si!",
                        "Sunutaba Abulumba! Umba! Si!"
                    ],
                )?;
                ctx.next()?;
                if ctx.call(Function::Rand, args![1, 10])? == 1 {
                    ctx.lines_as(
                        "Puchuchartan",
                        args![
                            "I guess my power was not enough.",
                            "The natural power I gathered with",
                            "my spell lost focus and was scattered..."
                        ],
                    )?;
                    ctx.call(Function::DelItem, args![l_consume.clone(), l_amount.clone()])?;
                    ctx.items().give(910, 1)?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Puchuchartan",
                        args![
                            "It seems the spirits of nature",
                            "were not in harmony at the moment.",
                            "However, if you come back later,",
                            "I will try to help you. Of course,",
                            "there will still be the same possibility that I may fail."
                        ],
                    )?;
                    return ctx.close();
                } else {
                    ctx.lines_as(
                        "Puchuchartan",
                        args![
                            "Here's the enchanted stone you",
                            "wished to have. I created this",
                            "with a lot of effort, so make",
                            "good use of it."
                        ],
                    )?;
                    ctx.call(Function::DelItem, args![l_consume.clone(), l_amount.clone()])?;
                    ctx.call(Function::GetItem, args![l_success.clone(), 1])?;
                    return ctx.close();
                }
            } else {
                ctx.mes("[Puchuchartan]")?;
                match l_success.number()? {
                    990 => {
                        ctx.lines(args![
                            "Fire property...",
                            "I will need natural",
                            "objects that are filled with",
                            "the spirit of fire."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Puchuchartan",
                            args![
                                "Scorpion which endures the",
                                "blazing heat of the desert",
                                "is brimming with fire energy.",
                                "I need ^3377FF20 Scorpion Tails^000000."
                            ],
                        )?;
                    }
                    991 => {
                        ctx.lines(args![
                            "Water property...",
                            "I will need natural",
                            "objects that are filled with",
                            "the spirit of water."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Puchuchartan",
                            args![
                                "Ambernite...the spirit of",
                                "water is contained within its",
                                "protective shell...",
                                "I will need ^3377FF20 Snail's Shell.^000000"
                            ],
                        )?;
                    }
                    992 => {
                        ctx.lines(args![
                            "Wind property...",
                            "I will need natural",
                            "objects that are filled with",
                            "the spirit of wind."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Puchuchartan",
                            args![
                                "Stainer...the beetle",
                                "that flies through the sky",
                                "has the wind's spirit.",
                                "I need ^3377FF25 Rainbow Shells^000000."
                            ],
                        )?;
                    }
                    993 => {
                        ctx.lines(args![
                            "Earth property...I need natural",
                            "objects that are filled with",
                            "the spirit of the Earth."
                        ])?;
                        ctx.next()?;
                        ctx.lines_as(
                            "Puchuchartan",
                            args![
                                "Horn...that dwells in the forest",
                                "is filled with the spirits of",
                                "earth and wood...Horn...",
                                "I need ^3377FF15 Horn^000000 from Horns."
                            ],
                        )?;
                    }
                    _ => {}
                }
                ctx.lines_as(
                    "Puchuchartan",
                    args!["That's all I need...", "Come back when", "you're ready.", "I will be here."],
                )?;
                return ctx.close();
            }
        }
        1 => {
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "Do you wish to divide a pure",
                    "enchanted stone into rough ones?",
                    "Which property do you want to",
                    "divide? Earth, Water, Fire, Wind... ",
                    "Choose one."
                ],
            )?;
            ctx.next()?;
            match ctx.menu(&["Earth", "Water", "Fire", "Wind"])? {
                0 => {
                    l_divide = Val::from(997);
                }
                1 => {
                    l_divide = Val::from(995);
                }
                2 => {
                    l_divide = Val::from(994);
                }
                3 => {
                    l_divide = Val::from(996);
                }
                _ => {}
            }
            ctx.lines_as(
                "Puchuchartan",
                args![
                    "Please enter the",
                    "number of enchanted",
                    "stones that you wish",
                    "to divide. I can only",
                    "divide up to 10 at a time."
                ],
            )?;
            ctx.next()?;
            let (input, _) = runtime::input_number(ctx, Some(0), Some(11))?;
            l_input = input;
            if l_input.number()? > 0 && l_input.number()? < 11 {
                if ctx.call(Function::CountItem, args![l_divide.clone()])?.number()? >= l_input.number()? {
                    if ctx.call(Function::CheckWeight, args![908, l_input.number()? * 30])? == 0 {
                        ctx.lines_as(
                            "Puchuchartan",
                            args![
                                "You're carrying too",
                                "many items right now.",
                                "Put some of your stuff",
                                "in Kafra Storage, and then",
                                "come back to me, okay?"
                            ],
                        )?;
                        return ctx.close();
                    }
                    ctx.lines_as(
                        "Puchuchartan",
                        args![
                            "I'll try to revert these",
                            "enchanted stones to their",
                            "rough forms. Enter a number",
                            "from 1 to 9, or enter 0 if",
                            "you decide to cancel."
                        ],
                    )?;
                    ctx.next()?;
                    'l8: loop {
                        let (input, _) = runtime::input_number(ctx, Some(0), Some(10))?;
                        l_input2 = input;
                        if l_input2 == 0 {
                            ctx.lines_as(
                                "Puchuchartan",
                                args![
                                    "You want to cancel?",
                                    "Well, if you change your",
                                    "mind, feel free to come",
                                    "ask me to help at any time."
                                ],
                            )?;
                            return ctx.close();
                        } else if l_input2.number()? > 9 {
                            ctx.lines_as("Puchuchartan", args!["Hm? You need to enter", "a number from 1 to 9."])?;
                            ctx.next()?;
                        } else {
                            break 'l8;
                        }
                    }
                    ctx.lines_as(
                        "Puchuchartan",
                        args!["I will now chant the", "sacred words. Remember", "the number you entered!"],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Puchuchartan",
                        args![
                            "Umba Umba Kalapum! Umba! Ta!",
                            "Lukura Ukulele Um! Umba! Ka!",
                            "Abulaka Tabulakan! Umba! La!",
                            "Ombaludu Zan Kunu! Umba! Ku!",
                            "Kum Tum Lakulakun! Umba! Ha!"
                        ],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Puchuchartan",
                        args![
                            "Here's the rough enchanted stones",
                            "you wished to have. I created this",
                            "with a lot of effort, so make good",
                            "use of them."
                        ],
                    )?;
                    'l9: loop {
                        if l_sha_man.loosely_equals(&l_input) {
                            break 'l9;
                        } else {
                            l_shaman_max = l_shaman_max.clone() + ctx.call(Function::Rand, args![6, 10])?;
                            l_sha_man = l_sha_man.clone() + Val::from(1);
                        }
                    }
                    match l_divide.number()? {
                        994 => {
                            ctx.call(Function::GetItem, args![990, l_shaman_max.clone()])?;
                        }
                        995 => {
                            ctx.call(Function::GetItem, args![991, l_shaman_max.clone()])?;
                        }
                        996 => {
                            ctx.call(Function::GetItem, args![992, l_shaman_max.clone()])?;
                        }
                        997 => {
                            ctx.call(Function::GetItem, args![993, l_shaman_max.clone()])?;
                        }
                        _ => {}
                    }
                    ctx.call(Function::DelItem, args![l_divide.clone(), l_input.clone()])?;
                    return ctx.close();
                } else {
                    ctx.lines_as("Puchuchartan", args!["So, you wish to have rough"])?;
                    match l_divide.number()? {
                        994 => {
                            ctx.lines(args![
                                "fire stones? Then I will need",
                                ((Val::from("you to bring ") + ctx.var("input_want").get()?) + Val::from(" pure fire stone."))
                            ])?;
                        }
                        995 => {
                            ctx.lines(args![
                                "water stones? Then I'll need",
                                ((Val::from("you to bring ") + ctx.var("input_want").get()?) + Val::from(" pure water stone."))
                            ])?;
                        }
                        996 => {
                            ctx.lines(args![
                                "wind stones? Then I will need",
                                ((Val::from("you to bring ") + ctx.var("input_want").get()?) + Val::from(" pure wind stone."))
                            ])?;
                        }
                        997 => {
                            ctx.lines(args![
                                "earth stones? Then I'll need",
                                ((Val::from("you to bring ") + l_input.clone()) + Val::from(" pure earth stone."))
                            ])?;
                        }
                        _ => {}
                    }
                    ctx.lines(args![
                        ((((Val::from("^3377FF") + l_input.clone()) + Val::from(" "))
                            + ctx.call(Function::GetItemName, args![l_divide.clone()])?)
                            + Val::from("^000000."))
                    ])?;
                    ctx.next()?;
                    ctx.lines_as(
                        "Puchuchartan",
                        args!["That's all I need...", "Come back when", "you're ready.", "I will be here."],
                    )?;
                    return ctx.close();
                }
            } else {
                ctx.lines_as("Puchuchartan", args!["Hm? You need to enter", "a number from 1 to 10."])?;
                return ctx.close();
            }
        }
        2 => {
            ctx.lines_as("Puchuchartan", args!["I see. It's your call.", "Come back when you need me."])?;
            return ctx.close();
        }
        _ => {}
    }
    Ok(())
}

pub fn skulldoor(ctx: &Ctx) -> Script {
    let mut l_insert = Val::from(0);
    let mut l_insert2 = Val::from(0);
    let mut l_skull = Val::from(0);
    let mut l_skulldoor = Val::from(0);
    let mut l_skullopen = Val::from(0);
    if ctx.var("event_umbala").get()?.number()? >= 7 {
        ctx.warp("um_in", 32, 71)?;
        return ctx.end();
    } else {
        ctx.lines(args![
            "^3355FFA human skull disturbingly",
            "hangs beside the door. The door is",
            "locked tight, so you can't get in.",
            "As you peer through the keyhole,",
            "you can see somebody moving inside the room.^000000"
        ])?;
        ctx.next()?;
        if ctx.menu(&["Examine the skull.", "Quit."])? == 0 {
            ctx.lines(args![
                "^3355FFYou see that the eye sockets",
                "of the skull are empty.",
                "How peculiar...",
                "It seems that Gemstones",
                "would fit perfectly inside of",
                "them.^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FFYou see the left eye socket of the",
                "skull. What do you want to do?^000000"
            ])?;
            ctx.next()?;
            match ctx.menu(&[
                "Leave it as it is.",
                "Insert a Blue Gemstone.",
                "Insert a Yellow Gemstone.",
                "Insert a Red Gemstone.",
            ])? {
                0 => {
                    ctx.mes("^3355FFYou left the eye socket as it was.^000000")?;
                    ctx.next()?;
                }
                1 => {
                    l_insert = Val::from(717);
                }
                2 => {
                    l_insert = Val::from(715);
                }
                3 => {
                    l_insert = Val::from(716);
                }
                _ => {}
            }
            if l_insert.is_true() {
                if ctx.call(Function::CountItem, args![l_insert.clone()])?.number()? > 0 {
                    ctx.lines(args![
                        ((Val::from("^3355FFYou inserted a ") + ctx.call(Function::GetItemName, args![l_insert.clone()])?) + Val::from("")),
                        "into the eye socket.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe gemstone rolled back out of",
                        "the mouth of the skull.^000000"
                    ])?;
                    l_skulldoor = l_skulldoor.clone() + Val::from(1);
                    match l_insert.number()? {
                        715 => {
                            l_skull = Val::from(2);
                        }
                        716 => {
                            l_skull = Val::from(3);
                        }
                        717 => {
                            l_skull = Val::from(1);
                        }
                        _ => {}
                    }
                    ctx.call(Function::DelItem, args![l_insert.clone(), 1])?;
                    ctx.call(Function::GetItem, args![l_insert.clone(), 1])?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![
                        ((Val::from("^3355FFYou forgot to carry ") + ctx.call(Function::GetItemName, args![l_insert.clone()])?)
                            + Val::from("")),
                        "with you. So you couldn't do what you",
                        "had intended.^000000"
                    ])?;
                    ctx.next()?;
                }
            }
            ctx.lines(args![
                "^3355FFYou see the right eye socket of",
                "the skull. What do you want to do?^000000"
            ])?;
            ctx.next()?;
            match ctx.menu(&[
                "Leave it as it is.",
                "Insert a Blue Gemstone.",
                "Insert a Yellow Gemstone.",
                "Insert a Red Gemstone.",
            ])? {
                0 => {
                    ctx.mes("^3355FFYou left the eye socket as it was.^000000")?;
                    ctx.next()?;
                }
                1 => {
                    l_insert2 = Val::from(717);
                }
                2 => {
                    l_insert2 = Val::from(715);
                }
                3 => {
                    l_insert2 = Val::from(716);
                }
                _ => {}
            }
            if l_insert2.is_true() {
                if ctx.call(Function::CountItem, args![l_insert2.clone()])?.number()? > 0 {
                    ctx.lines(args![
                        ((Val::from("^3355FFYou inserted a ") + ctx.call(Function::GetItemName, args![l_insert2.clone()])?)
                            + Val::from("")),
                        "into the eye socket.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.lines(args![
                        "^3355FFThe gemstone rolled back out of",
                        "the mouth of the skull.^000000"
                    ])?;
                    if l_insert2.loosely_equals(&l_insert) {
                        l_skulldoor = l_skulldoor.clone() + Val::from(1);
                    } else {
                        l_skulldoor = l_skulldoor.clone() + Val::from(2);
                    }
                    ctx.call(Function::DelItem, args![l_insert2.clone(), 1])?;
                    ctx.call(Function::GetItem, args![l_insert2.clone(), 1])?;
                    ctx.next()?;
                } else {
                    ctx.lines(args![
                        ((Val::from("^3355FFYou forgot to carry ") + ctx.call(Function::GetItemName, args![l_insert2.clone()])?)
                            + Val::from("")),
                        "with you. So you couldn't do what you",
                        "had intended.^000000"
                    ])?;
                    ctx.next()?;
                }
            }
            ctx.mes("^3355FF..............................^000000")?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FF..............................",
                "..............................^000000"
            ])?;
            ctx.next()?;
            ctx.lines(args![
                "^3355FF..............................",
                "..............................",
                "..............................^000000"
            ])?;
            ctx.next()?;
            match l_skulldoor.number()? {
                3 => {
                    if ctx.call(Function::Rand, args![1, 4])? != 1 {
                        l_skullopen = Val::from(1);
                    }
                }
                2 => {
                    if ctx.call(Function::Rand, args![1, 2])? == 2 {
                        l_skullopen = Val::from(1);
                    }
                }
                1 => {
                    if ctx.call(Function::Rand, args![1, 4])? == 1 {
                        l_skullopen = Val::from(1);
                    }
                }
                _ => {}
            }
            if l_skullopen == 0 {
                ctx.lines(args![
                    "^3355FFNothing happened.",
                    "You have the feeling that the",
                    "skull is grinning at you. But...",
                    "It's probably just a trick of the light.^000000"
                ])?;
                return ctx.close();
            } else {
                ctx.lines(args![
                    "^3355FFSuddenly, a clicking sound comes",
                    "from the skull's eye sockets and",
                    "the door opens. Before you know",
                    "it, you walk inside as if guided",
                    "by an unseen force...^000000"
                ])?;
                ctx.close_window()?;
                ctx.warp("um_in", 32, 71)?;
                return ctx.end();
            }
        }
        ctx.lines(args![
            "^3355FFYou decided to pass by the door.",
            "It looks like it might be too hard to open.^000000"
        ])?;
        return ctx.close();
    }
}

pub fn phrenetan(ctx: &Ctx) -> Script {
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.lines_as(
            "Phrenetan",
            args![
                "I am so sick and tired of",
                "my husband!! It's like he",
                "flirts with every girl",
                "in the village!"
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Phrenetan",
            args!["If I see him flirting with", "women again...I swear...", "I will show him hell!!"],
        )?;
        return ctx.close();
    }
    if ctx.var("BaseJob").get()? == constants::JOB_NOVICE && ctx.var("Upper").get()? != 2 {
        ctx.lines_as(
            "Phrenetan",
            args!["Umba~ umbaumbah!", "Umbah woomumum!", "Umbah woomum umbabah!"],
        )?;
        return ctx.close();
    }
    if ctx.var("um_wind").get()?.number()? <= 3 && (ctx.var("misc_quest").get()?.number()? & 32768) == 0 {
        if ctx.var("um_wind").get()? == 0 {
            ctx.var("um_wind").set(Val::from(1))?;
        }
        ctx.call(Function::Emotion, args![constants::ET_FRET])?;
        ctx.lines_as("Phrenetan", args!["Umbaumbah wooga wooga", "Umbaumbabah babababah!", "Umbaum!"])?;
        ctx.next()?;
        ctx.call(Function::Emotion, args![constants::ET_O])?;
        ctx.lines_as(
            "Phrenetan",
            args!["Umbah umbaumba umbah", "Umbabababah wooga woo!", "Wooga wooga umbabah umbaum!"],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, args![constants::ET_GO])?;
        return ctx.close();
    } else if ctx.var("um_wind").get()? == 6 || (ctx.var("misc_quest").get()?.number()? & 32768) != 0 {
        ctx.call(Function::Emotion, args![constants::ET_FRET])?;
        ctx.lines_as(
            "Phrenetan",
            args![
                "Umbabah! Umbaumbah.....",
                "Umbaum Umbaum Wooga wooga!",
                "Wooga umumum woombababap!!!!!"
            ],
        )?;
        ctx.next()?;
        ctx.call(Function::Emotion, args![constants::ET_SWEAT])?;
        return ctx.close();
    }
    return ctx.end();
}

pub fn umpokoriohtan(ctx: &Ctx) -> Script {
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        if ctx.call(Function::Rand, args![1, 3])? == 2 {
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "Hey there, cool cat.",
                    "Don't mind the wife...",
                    "Much as I love her,",
                    "I know my obligations, ya dig?"
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "If a man's got plenty, he's",
                    "got to share it with those",
                    "that got nothing to give."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "If a man's hands are good",
                    "at healin', he's got to use",
                    "those hands to help folks live."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "If a man's lips be good at",
                    "singin', he's got to croon the",
                    "songs we like to hear so much."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args!["But if sweet lovin' is golden,", "then baby...I got the Midas touch."],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "Ooh...!",
                    "...........",
                    "My back--!",
                    "Simmer down, baby, your turn is comin' in a minute~"
                ],
            )?;
            ctx.call(
                Function::Emotion,
                args![constants::ET_ROCK, ctx.call(Function::GetNpcId, args![0, "Phrenetan"])?],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_SWEAT])?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "Man...sometimes my wife can",
                    "be a lil' too rough, maybe",
                    "even hurtful. But that's cool...",
                    "it just means she's got fire."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "But someday, she'll have to",
                    "learn that I gots to share",
                    "this heart of mine with the ladies",
                    "who really need a dose of vitamin",
                    "lovin', ya dig? It's my obligation."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "Wainatan, Bertztan, Chabimatan...",
                    "Those pretty girls been waitin'",
                    "toooooo long. Don't worry,",
                    "big daddy's comin' soon."
                ],
            )?;
            ctx.next()?;
            ctx.lines_as(
                "Umpokoriohtan",
                args![
                    "OOOH~! Phrenetan!",
                    "Baby, why you gotta be rough?",
                    "Hit me gently, ya dig??",
                    "I don't mean to hurt you~"
                ],
            )?;
            ctx.call(
                Function::Emotion,
                args![constants::ET_HUK, ctx.call(Function::GetNpcId, args![0, "Phrenetan"])?],
            )?;
            ctx.call(Function::Emotion, args![constants::ET_KEK])?;
            return ctx.close();
        }
    }
    ctx.call(Function::Emotion, args![constants::ET_SMILE])?;
    ctx.lines_as(
        "Umpokoriohtan",
        args!["Umbaumbah...........", "Umbahwooga woogawoo!", "Umbah umumbabah umbawoo gaga."],
    )?;
    ctx.next()?;
    ctx.call(Function::Emotion, args![constants::ET_FRET])?;
    return ctx.close();
}

pub fn umpokoriohtan_oninit(ctx: &Ctx) -> Script {
    ctx.set_npc_visible("Umpokoriohtan", false)?;
    return ctx.end();
}

pub fn wainatan(ctx: &Ctx) -> Script {
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.lines_as(
            "Wainatan",
            args![
                "I am sick and tired of this guy",
                "who always appears at night and bugs the hell out of me..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Wainatan",
            args![
                "'Smooth operator that gets the",
                "job done?' Oh my god...!",
                "I hate him with a passion!",
                "I wish Umpokoriohtan would",
                "just drop dead."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("um_wind").get()? == 1 {
        if ctx.time_field(constants::DT_HOUR)? > 18 {
            ctx.var("um_wind").set(Val::from(2))?;
            ctx.call(Function::Emotion, args![constants::ET_FRET])?;
            ctx.lines_as(
                "Wainatan",
                args!["Umbaumbah umgagaga.", "Umbaumbawoogawoo gababah.", "Umbahumbabah gawoo."],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_O])?;
            ctx.lines_as(
                "Wainatan",
                args!["Wooga wooga woogagagah", "Wogagagah woogagagah", "Gawoo gawoo gah."],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Wainatan",
                args!["Umbabah! Umbaumbah wooga", "Woogawooga umbawooga umum.", "Umbabababababababababah."],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Wainatan",
            args![
                "Umbaumbah umbaumbah umbah",
                "Wooga wooga woogawooga wooga",
                "Umumumum umumumum umum."
            ],
        )?;
        return ctx.close();
    }
}

pub fn bertztan(ctx: &Ctx) -> Script {
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.lines_as(
            "Bertztan",
            args![
                "...*Sigh* That sicko",
                "Umpo-whatever! I told him",
                "I don't like him, but he",
                "just doesn't listen!",
                "I wish...I wish he would",
                "just disappear!"
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("um_wind").get()? == 2 {
        if ctx.time_field(constants::DT_HOUR)? > 18 {
            ctx.var("um_wind").set(Val::from(3))?;
            ctx.call(Function::Emotion, args![constants::ET_FRET])?;
            ctx.lines_as(
                "Bertztan",
                args!["Umbaumbah umgagaga.", "Umbaumbawoogawoo gababah.", "Umbahumbabah gawoo."],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_O])?;
            ctx.lines_as(
                "Bertztan",
                args![
                    "Wooga umbar umbar umbah!",
                    "Umbar woogagaga woo! Woo! Woo!",
                    "Wooga~ wooga~ Woo woo woo umbar."
                ],
            )?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Bertztan",
                args!["Umbar woogaumbarumbah um!", "Um~ wooga wooga umbarum.", "Umbah...wooum."],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Bertztan",
            args!["Umbar wooga umbar umbah um!", "Um~ woogawooga umbar um.", "Umbah...wooum."],
        )?;
        return ctx.close();
    }
}

pub fn chabimatan(ctx: &Ctx) -> Script {
    if ctx.var("event_umbala").get()?.number()? >= 3 {
        ctx.lines_as(
            "Chabimatan",
            args![
                "...*Sigh* Umpokoriohtan seems",
                "to be married. I have no",
                "idea why he still flirts",
                "with other women. Maybe he's",
                "not very mature, or he's",
                "irresponsible..."
            ],
        )?;
        ctx.next()?;
        ctx.lines_as(
            "Chabimatan",
            args![
                "Well, whatever he is,",
                "he's certainly not",
                "romantic. Those pick-up",
                "lines of his could",
                "some work, maybe",
                "even some clean up."
            ],
        )?;
        return ctx.close();
    }
    if ctx.var("um_wind").get()? == 3 {
        if ctx.time_field(constants::DT_HOUR)? > 18 {
            ctx.var("um_wind").set(Val::from(4))?;
            ctx.call(Function::Emotion, args![constants::ET_FRET])?;
            ctx.lines_as(
                "Chabimatan",
                args![
                    "Umbabah umbarbar woogawooga um",
                    "Umbabah umbarbar woogawooga umbah",
                    "Umumum! Wooga!"
                ],
            )?;
            ctx.next()?;
            ctx.call(Function::Emotion, args![constants::ET_O])?;
            ctx.lines_as(
                "Chabimatan",
                args!["Umbabah~~~~~~~", "Woogawooga umbar umbar woo!", "Wooga umbar woogawoogagah."],
            )?;
            ctx.set_npc_visible("Umpokoriohtan", true)?;
            ctx.set_npc_visible("#!@#$%", true)?;
            return ctx.close();
        } else {
            ctx.lines_as(
                "Chabimatan",
                args![
                    "Umbabah~~~~~~~",
                    "Woogawooga umbar umbar woo",
                    "woo woo! Nook nook~",
                    "Wooga umbar wooga umbar",
                    "wooga woogagah."
                ],
            )?;
            return ctx.close();
        }
    } else {
        ctx.lines_as(
            "Chabimatan",
            args![
                "Umbabah~~~~~~~",
                "Woogawooga umbar umbar",
                "woo woo woo nook nook.",
                "Wooga umbar wooga umbar",
                "wooga woogagah."
            ],
        )?;
        return ctx.close();
    }
}

#[derive(Clone, Copy, Debug)]
enum ScriptStep {
    Start,
    OnInit,
    OnTouch,
}

fn script_run(ctx: &Ctx, mut step: ScriptStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            ScriptStep::Start => {
                step = ScriptStep::OnInit;
                continue 'machine;
            }
            ScriptStep::OnInit => {
                ctx.set_npc_visible("#!@#$%", false)?;
                return Err(Stop::End);
            }
            ScriptStep::OnTouch => {
                if ctx.var("um_wind").get()? == 4 {
                    ctx.var("um_wind").set(Val::from(5))?;
                    ctx.lines(args![
                        "^3355FFAs you enter the house",
                        "you happen to witness",
                        "Phrenetan beating a guy",
                        "mercilessly.^000000"
                    ])?;
                    ctx.next()?;
                    ctx.call(Function::Emotion, args![constants::ET_FRET])?;
                    ctx.lines_as(
                        "Phrenetan",
                        args!["Umbaumbaumbaumbah!", "Umbaumbahumbah!!", "Umbaumbahumbah!!!!!!"],
                    )?;
                    ctx.next()?;
                    ctx.lines(args!["^3355FFYou were kicked out of the house", "by Phrenetan.^000000"])?;
                    ctx.next()?;
                    ctx.set_npc_visible("#unpc", true)?;
                    ctx.set_npc_visible("#!@#$%", false)?;
                    ctx.warp("umbala", 94, 181)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn script(ctx: &Ctx) -> Script {
    script_run(ctx, ScriptStep::Start, Vec::new()).map(|_| ())
}

pub fn script_oninit(ctx: &Ctx) -> Script {
    script_run(ctx, ScriptStep::OnInit, Vec::new()).map(|_| ())
}

pub fn script_ontouch(ctx: &Ctx) -> Script {
    script_run(ctx, ScriptStep::OnTouch, Vec::new()).map(|_| ())
}

#[derive(Clone, Copy, Debug)]
enum UnpcStep {
    Start,
    OnInit,
    OnTouch,
}

fn unpc_run(ctx: &Ctx, mut step: UnpcStep, args: Vec<Val>) -> Result<Val, Stop> {
    'machine: loop {
        match step {
            UnpcStep::Start => {
                step = UnpcStep::OnInit;
                continue 'machine;
            }
            UnpcStep::OnInit => {
                ctx.set_npc_visible("#unpc", false)?;
                return Err(Stop::End);
            }
            UnpcStep::OnTouch => {
                if ctx.var("um_wind").get()? == 5 {
                    ctx.lines(args![
                        "^3355FFAs you realized what happened",
                        "after being kicked out of the",
                        "house, you see a leaf on the",
                        "ground near where you're standing.^000000"
                    ])?;
                    ctx.next()?;
                    if ctx.menu(&["Take it.", "Leave it."])? == 0 {
                        ctx.close_window()?;
                        ctx.var("um_wind").set(Val::from(0))?;
                        ctx.var("misc_quest")
                            .set(Val::from(ctx.var("misc_quest").get()?.number()? | 32768))?;
                        ctx.items().give(610, 1)?;
                        ctx.set_npc_visible("#unpc", false)?;
                        return Err(Stop::End);
                    }
                    ctx.var("um_wind").set(Val::from(0))?;
                    ctx.var("misc_quest")
                        .set(Val::from(ctx.var("misc_quest").get()?.number()? | 32768))?;
                    ctx.lines_as(
                        ctx.player().name()?,
                        args!["I am not supposed to take", "what may belong to other people."],
                    )?;
                    ctx.next()?;
                    ctx.lines_as(ctx.player().name()?, args!["Yeah, I'm a such good person."])?;
                    ctx.close_window()?;
                    ctx.set_npc_visible("#unpc", false)?;
                }
                return Err(Stop::End);
            }
        }
    }
}

pub fn unpc(ctx: &Ctx) -> Script {
    unpc_run(ctx, UnpcStep::Start, Vec::new()).map(|_| ())
}

pub fn unpc_oninit(ctx: &Ctx) -> Script {
    unpc_run(ctx, UnpcStep::OnInit, Vec::new()).map(|_| ())
}

pub fn unpc_ontouch(ctx: &Ctx) -> Script {
    unpc_run(ctx, UnpcStep::OnTouch, Vec::new()).map(|_| ())
}
